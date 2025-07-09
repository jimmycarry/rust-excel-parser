use crate::error::{DocParserError, Result};
use crate::parser::DocData;
use std::path::Path;

/// 解析老式DOC文件
/// 注意: 此功能需要 "legacy-doc" feature 启用
pub fn parse_doc<P: AsRef<Path>>(file_path: P) -> Result<DocData> {
    let _file_path = file_path.as_ref();
    
    #[cfg(feature = "legacy-doc")]
    {
        // 使用dotext库提取DOC文件文本
        use dotext::Docx;
        use crate::parser::{DocMetadata, DocSection, SectionType};
        
        let mut file = Docx::open(file_path)
            .map_err(|e| DocParserError::DocParsing {
                file: file_path.as_ref().display().to_string(),
                details: format!("Failed to open DOC file: {:?}", e),
            })?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| DocParserError::TextExtraction {
                file: file_path.as_ref().display().to_string(),
                details: format!("Failed to extract text: {:?}", e),
            })?;
        
        // 清理文本
        let cleaned_content = crate::parser::text_extractor::TextCleaner::clean_text(&content);
        
        // 简单的段落分割
        let sections: Vec<DocSection> = cleaned_content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| DocSection {
                section_type: SectionType::Paragraph,
                content: line.to_string(),
                level: None,
                formatting: None,
            })
            .collect();
        
        let word_count = cleaned_content.split_whitespace().count();
        let character_count = cleaned_content.chars().count();
        let paragraph_count = sections.len();
        
        let metadata = DocMetadata {
            title: None,
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
            content: content.clone(),
            metadata,
            sections,
            raw_text: cleaned_content,
        })
    }
    
    #[cfg(not(feature = "legacy-doc"))]
    {
        Err(DocParserError::UnsupportedFormat {
            format: "doc".to_string(),
            file: file_path.as_ref().display().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_format_not_supported_without_feature() {
        #[cfg(not(feature = "legacy-doc"))]
        {
            let result = parse_doc("test.doc");
            assert!(matches!(result, Err(DocParserError::UnsupportedFormat { .. })));
        }
    }
}