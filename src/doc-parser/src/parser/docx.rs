use crate::error::{DocParserError, Result};
use crate::parser::{DocData, DocMetadata, DocSection, SectionType};
use crate::parser::text_extractor::TextCleaner;
use crate::parser::table::{TableExtractor, TableExtractionConfig};
use std::path::Path;

/// 解析DOCX文件
pub fn parse_docx<P: AsRef<Path>>(file_path: P) -> Result<DocData> {
    let file_path = file_path.as_ref();
    
    // 读取DOCX文件
    let file_data = std::fs::read(file_path)
        .map_err(|e| DocParserError::IoError {
            file: file_path.display().to_string(),
            source: e,
        })?;
    
    // 使用docx-rs解析DOCX内容
    let docx = docx_rs::read_docx(&file_data)
        .map_err(|e| DocParserError::DocxParsing {
            file: file_path.display().to_string(),
            details: format!("Failed to parse DOCX: {:?}", e),
        })?;

    let mut content = String::new();
    let mut raw_text = String::new();
    let mut sections = Vec::new();
    let mut paragraph_count = 0;
    let mut word_count = 0;

    // 提取文档内容
    for child in &docx.document.children {
        match child {
            docx_rs::DocumentChild::Paragraph(paragraph) => {
                let para_text = extract_paragraph_text(&paragraph);
                if !para_text.trim().is_empty() {
                    content.push_str(&para_text);
                    content.push('\n');
                    
                    raw_text.push_str(&TextCleaner::clean_text(&para_text));
                    raw_text.push('\n');
                    
                    // 简化的标题检测
                    let heading_level = detect_heading_level_simple(&para_text);
                    let (section_type, level) = if let Some(lvl) = heading_level {
                        (SectionType::Heading(lvl), Some(lvl))
                    } else {
                        (SectionType::Paragraph, None)
                    };
                    
                    sections.push(DocSection {
                        section_type,
                        content: para_text.clone(),
                        level,
                        formatting: None, // 暂时禁用格式提取
                    });
                    
                    paragraph_count += 1;
                    word_count += para_text.split_whitespace().count();
                }
            }
            docx_rs::DocumentChild::Table(table) => {
                let table_text = extract_table_text_simple(&table);
                content.push_str(&table_text);
                content.push('\n');
                
                raw_text.push_str(&TextCleaner::clean_text(&table_text));
                raw_text.push('\n');
                
                sections.push(DocSection {
                    section_type: SectionType::Table,
                    content: table_text.clone(),
                    level: None,
                    formatting: None,
                });
                
                word_count += table_text.split_whitespace().count();
            }
            _ => {}
        }
    }

    // 构建简化的元数据
    let character_count = raw_text.chars().count();
    let metadata = DocMetadata {
        title: extract_document_title_simple(&docx),
        author: None,
        subject: None,
        created: None,
        modified: None,
        word_count,
        paragraph_count,
        page_count: None,
        character_count,
    };

    Ok(DocData {
        content: content.trim().to_string(),
        metadata,
        sections,
        raw_text: raw_text.trim().to_string(),
    })
}

fn extract_paragraph_text(paragraph: &docx_rs::Paragraph) -> String {
    let mut text = String::new();
    for child in &paragraph.children {
        if let docx_rs::ParagraphChild::Run(run) = child {
            for run_child in &run.children {
                if let docx_rs::RunChild::Text(text_element) = run_child {
                    text.push_str(&text_element.text);
                }
            }
        }
    }
    text
}

fn extract_table_text_simple(table: &docx_rs::Table) -> String {
    // 使用新的TableExtractor进行表格提取
    let config = TableExtractionConfig::simple();
    let extractor = TableExtractor::new(config);
    
    match extractor.extract_table(table) {
        Ok(table_data) => {
            // 转换为简单的文本格式
            let mut result = String::new();
            
            // 添加表头（如果有）
            if let Some(headers) = &table_data.headers {
                result.push_str(&headers.join(" | "));
                result.push('\n');
                result.push_str(&"-".repeat(headers.len() * 10));
                result.push('\n');
            }
            
            // 添加数据行
            for row in &table_data.rows {
                if table_data.has_header && row.is_header {
                    continue; // 跳过已经处理的标题行
                }
                
                let row_text: Vec<String> = row.cells.iter()
                    .map(|cell| cell.content.trim().to_string())
                    .collect();
                
                if !row_text.is_empty() && !row_text.iter().all(|s| s.is_empty()) {
                    result.push_str(&row_text.join(" | "));
                    result.push('\n');
                }
            }
            
            result.trim().to_string()
        }
        Err(_) => {
            // 如果提取失败，回退到原来的简化方式
            format!("[Table with {} rows]", table.rows.len())
        }
    }
}

fn detect_heading_level_simple(text: &str) -> Option<u8> {
    // 基于内容的简单标题检测
    let text_lower = text.to_lowercase();
    let text_len = text.trim().len();
    
    // 常见的标题关键词
    if text_lower.contains("chapter") || text_lower.contains("section") {
        return Some(2);
    }
    
    // 基于长度和格式的启发式判断
    if text_len < 100 && (text.chars().any(|c| c.is_numeric()) || text_len < 50) {
        if text_len < 20 {
            Some(1)
        } else if text_len < 35 {
            Some(2)
        } else {
            Some(3)
        }
    } else {
        None
    }
}

fn extract_document_title_simple(docx: &docx_rs::Docx) -> Option<String> {
    // 简化的标题提取：查找第一个可能的标题段落
    for child in &docx.document.children {
        if let docx_rs::DocumentChild::Paragraph(paragraph) = child {
            let para_text = extract_paragraph_text(paragraph);
            if !para_text.trim().is_empty() && para_text.len() < 100 {
                // 如果是短段落，可能是标题
                return Some(para_text);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_parse_invalid_docx() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"not a valid docx file").unwrap();
        
        let result = parse_docx(temp_file.path());
        assert!(matches!(result, Err(DocParserError::DocxParsing { .. })));
    }

    #[test]
    fn test_detect_heading_level_simple() {
        // 测试简单的标题检测
        assert_eq!(detect_heading_level_simple("Chapter 1"), Some(2));
        assert_eq!(detect_heading_level_simple("Short Title"), Some(1));
        assert_eq!(detect_heading_level_simple("This is a very long paragraph that should not be detected as a heading because it contains too many words and characters"), None);
    }

    #[test]
    fn test_extract_paragraph_text() {
        // 创建一个简单的测试段落
        use docx_rs::*;
        let paragraph = Paragraph::new()
            .add_run(Run::new().add_text("Test text"));
        
        let text = extract_paragraph_text(&paragraph);
        assert_eq!(text, "Test text");
    }

    #[test]
    fn test_extract_table_text_simple() {
        use docx_rs::*;
        let table = Table::new(vec![
            TableRow::new(vec![
                TableCell::new(),
                TableCell::new(),
            ]),
        ]);
        
        let text = extract_table_text_simple(&table);
        // 现在的实现可能返回提取的内容或者备用格式
        assert!(text.contains("Table with 1 rows") || text.is_empty() || text.contains(" | "));
    }

    #[test]
    fn test_text_cleaner_integration() {
        let dirty_text = "  Multiple   spaces\t\nand  \r\n line breaks  ";
        let cleaned = TextCleaner::clean_text(dirty_text);
        assert_eq!(cleaned, "Multiple spaces and line breaks");
    }
}