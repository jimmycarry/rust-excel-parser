use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{DocData, DocSection, SectionType};
use std::io::Write;

pub struct MarkdownOutput {
    preserve_structure: bool,
    include_metadata: bool,
}

impl MarkdownOutput {
    pub fn new(preserve_structure: bool, include_metadata: bool) -> Self {
        Self {
            preserve_structure,
            include_metadata,
        }
    }
}

impl OutputWriter for MarkdownOutput {
    fn write_doc_data<W: Write>(&self, data: &DocData, writer: &mut W) -> Result<()> {
        if self.include_metadata {
            write_metadata_as_frontmatter(writer, &data.metadata)?;
            writeln!(writer)?;
        }

        if self.preserve_structure {
            self.write_sections(&data.sections, writer)?;
        } else {
            // 简单的文本到Markdown转换
            for line in data.raw_text.lines() {
                if !line.trim().is_empty() {
                    writeln!(writer, "{}\n", line)?;
                }
            }
        }

        Ok(())
    }

    fn write_sections<W: Write>(&self, sections: &[DocSection], writer: &mut W) -> Result<()> {
        for section in sections {
            let markdown_content = convert_section_to_markdown(section);
            writeln!(writer, "{}", markdown_content)?;
            
            // 在某些类型的内容后添加额外的换行
            match section.section_type {
                SectionType::Heading(_) | SectionType::Table => {
                    writeln!(writer)?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

fn write_metadata_as_frontmatter<W: Write>(writer: &mut W, metadata: &crate::parser::DocMetadata) -> Result<()> {
    writeln!(writer, "---")?;
    
    if let Some(title) = &metadata.title {
        writeln!(writer, "title: \"{}\"", escape_yaml_string(title))?;
    }
    if let Some(author) = &metadata.author {
        writeln!(writer, "author: \"{}\"", escape_yaml_string(author))?;
    }
    if let Some(subject) = &metadata.subject {
        writeln!(writer, "subject: \"{}\"", escape_yaml_string(subject))?;
    }
    if let Some(created) = &metadata.created {
        writeln!(writer, "created: \"{}\"", created)?;
    }
    if let Some(modified) = &metadata.modified {
        writeln!(writer, "modified: \"{}\"", modified)?;
    }
    
    writeln!(writer, "word_count: {}", metadata.word_count)?;
    writeln!(writer, "paragraph_count: {}", metadata.paragraph_count)?;
    writeln!(writer, "character_count: {}", metadata.character_count)?;
    
    if let Some(page_count) = metadata.page_count {
        writeln!(writer, "page_count: {}", page_count)?;
    }
    
    writeln!(writer, "---")?;
    
    Ok(())
}

fn convert_section_to_markdown(section: &DocSection) -> String {
    match &section.section_type {
        SectionType::Heading(level) => {
            format!("{} {}", "#".repeat(*level as usize), section.content)
        }
        SectionType::List => {
            format!("- {}", section.content)
        }
        SectionType::Table => {
            convert_table_to_markdown(&section.content)
        }
        SectionType::Image => {
            format!("![图片]({})", section.content)
        }
        SectionType::Hyperlink => {
            // 假设内容格式为 "text|url"
            if let Some((text, url)) = section.content.split_once('|') {
                format!("[{}]({})", text.trim(), url.trim())
            } else {
                format!("[{}]({})", section.content, section.content)
            }
        }
        SectionType::Header => {
            format!("> **页眉**: {}", section.content)
        }
        SectionType::Footer => {
            format!("> **页脚**: {}", section.content)
        }
        SectionType::Paragraph => {
            // 检查是否包含格式化信息
            if let Some(formatting) = &section.formatting {
                apply_markdown_formatting(&section.content, formatting)
            } else {
                section.content.clone()
            }
        }
    }
}

fn convert_table_to_markdown(table_content: &str) -> String {
    let lines: Vec<&str> = table_content.lines().collect();
    if lines.is_empty() {
        return String::new();
    }

    let mut markdown = String::new();
    
    for (i, line) in lines.iter().enumerate() {
        let cells: Vec<&str> = line.split('\t').collect();
        let row = format!("| {} |", cells.join(" | "));
        markdown.push_str(&row);
        markdown.push('\n');
        
        // 在第一行后添加分隔符
        if i == 0 && lines.len() > 1 {
            let separator = format!("|{}|", " --- |".repeat(cells.len()));
            markdown.push_str(&separator);
            markdown.push('\n');
        }
    }
    
    markdown
}

fn apply_markdown_formatting(text: &str, formatting: &crate::parser::FormatInfo) -> String {
    let mut result = text.to_string();
    
    if formatting.bold {
        result = format!("**{}**", result);
    }
    
    if formatting.italic {
        result = format!("*{}*", result);
    }
    
    if formatting.underline {
        result = format!("<u>{}</u>", result);
    }
    
    result
}

fn escape_yaml_string(s: &str) -> String {
    s.replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{DocMetadata, DocSection, SectionType, FormatInfo};

    #[test]
    fn test_markdown_output_creation() {
        let _output = MarkdownOutput::new(true, false);
        let _output_with_metadata = MarkdownOutput::new(true, true);
    }

    #[test]
    fn test_convert_heading_to_markdown() {
        let section = DocSection {
            section_type: SectionType::Heading(2),
            content: "Chapter 1".to_string(),
            level: Some(2),
            formatting: None,
        };

        let markdown = convert_section_to_markdown(&section);
        assert_eq!(markdown, "## Chapter 1");
    }

    #[test]
    fn test_convert_list_to_markdown() {
        let section = DocSection {
            section_type: SectionType::List,
            content: "First item".to_string(),
            level: None,
            formatting: None,
        };

        let markdown = convert_section_to_markdown(&section);
        assert_eq!(markdown, "- First item");
    }

    #[test]
    fn test_apply_formatting() {
        let formatting = FormatInfo {
            bold: true,
            italic: false,
            underline: false,
            font_size: None,
            font_family: None,
        };

        let result = apply_markdown_formatting("Bold text", &formatting);
        assert_eq!(result, "**Bold text**");
    }

    #[test]
    fn test_convert_table_to_markdown() {
        let table_content = "Header1\tHeader2\nRow1Col1\tRow1Col2\nRow2Col1\tRow2Col2";
        let markdown = convert_table_to_markdown(table_content);
        
        assert!(markdown.contains("| Header1 | Header2 |"));
        assert!(markdown.contains("| --- | --- |"));
        assert!(markdown.contains("| Row1Col1 | Row1Col2 |"));
    }

    #[test]
    fn test_escape_yaml_string() {
        let text = "Title with \"quotes\" and\nnewlines";
        let escaped = escape_yaml_string(text);
        assert_eq!(escaped, "Title with \\\"quotes\\\" and\\nnewlines");
    }

    #[test]
    fn test_full_doc_data_markdown() {
        let doc_data = DocData {
            content: "Test Document\n\nThis is a test paragraph.".to_string(),
            raw_text: "Test Document This is a test paragraph.".to_string(),
            metadata: DocMetadata {
                title: Some("Test Document".to_string()),
                author: Some("Test Author".to_string()),
                subject: Some("Testing".to_string()),
                created: Some("2023-01-01".to_string()),
                modified: Some("2023-01-02".to_string()),
                word_count: 6,
                paragraph_count: 2,
                page_count: Some(1),
                character_count: 42,
            },
            sections: vec![
                DocSection {
                    section_type: SectionType::Heading(1),
                    content: "Test Document".to_string(),
                    level: Some(1),
                    formatting: None,
                },
                DocSection {
                    section_type: SectionType::Paragraph,
                    content: "This is a test paragraph.".to_string(),
                    level: None,
                    formatting: None,
                },
                DocSection {
                    section_type: SectionType::Table,
                    content: "Header1\tHeader2\nCell1\tCell2".to_string(),
                    level: None,
                    formatting: None,
                },
            ],
        };

        let output = MarkdownOutput::new(true, true);
        let mut buffer = Vec::new();
        output.write_doc_data(&doc_data, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        
        // 验证Markdown包含元数据前置内容
        assert!(result.contains("---"));
        assert!(result.contains("title: \"Test Document\""));
        assert!(result.contains("author: \"Test Author\""));
        assert!(result.contains("word_count: 6"));
        
        // 验证Markdown格式化
        assert!(result.contains("# Test Document"));
        assert!(result.contains("This is a test paragraph."));
        assert!(result.contains("| Header1 | Header2 |"));
        assert!(result.contains("| Cell1 | Cell2 |"));
    }

    #[test]
    fn test_markdown_without_metadata() {
        let doc_data = DocData {
            content: "Simple content".to_string(),
            raw_text: "Simple content".to_string(),
            metadata: DocMetadata {
                title: None,
                author: None,
                subject: None,
                created: None,
                modified: None,
                word_count: 2,
                paragraph_count: 1,
                page_count: None,
                character_count: 14,
            },
            sections: vec![
                DocSection {
                    section_type: SectionType::Paragraph,
                    content: "Simple content".to_string(),
                    level: None,
                    formatting: None,
                },
            ],
        };

        let output = MarkdownOutput::new(true, false);
        let mut buffer = Vec::new();
        output.write_doc_data(&doc_data, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        
        // 不应该包含元数据前置内容
        assert!(!result.contains("---"));
        assert!(!result.contains("title:"));
        
        // 应该包含内容
        assert!(result.contains("Simple content"));
    }

    #[test]
    fn test_markdown_list_and_link() {
        let sections = vec![
            DocSection {
                section_type: SectionType::List,
                content: "First item".to_string(),
                level: None,
                formatting: None,
            },
            DocSection {
                section_type: SectionType::Hyperlink,
                content: "Click here|https://example.com".to_string(),
                level: None,
                formatting: None,
            },
        ];

        let output = MarkdownOutput::new(true, false);
        let mut buffer = Vec::new();
        output.write_sections(&sections, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        
        // 验证列表格式
        assert!(result.contains("- First item"));
        
        // 验证链接格式
        assert!(result.contains("[Click here](https://example.com)"));
    }
}