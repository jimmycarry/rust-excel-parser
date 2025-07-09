use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{DocData, DocSection, DocMetadata, SectionType, FormatInfo};
use serde::{Deserialize, Serialize};
use std::io::Write;

pub struct JsonOutput {
    pretty: bool,
    include_formatting: bool,
}

impl JsonOutput {
    pub fn new(pretty: bool, include_formatting: bool) -> Self {
        Self {
            pretty,
            include_formatting,
        }
    }
}

impl OutputWriter for JsonOutput {
    fn write_doc_data<W: Write>(&self, data: &DocData, writer: &mut W) -> Result<()> {
        let json_data = JsonDocData::from_doc_data(data, self.include_formatting);
        
        let json_string = if self.pretty {
            serde_json::to_string_pretty(&json_data)?
        } else {
            serde_json::to_string(&json_data)?
        };
        
        writeln!(writer, "{}", json_string)?;
        Ok(())
    }

    fn write_sections<W: Write>(&self, sections: &[DocSection], writer: &mut W) -> Result<()> {
        let json_sections: Vec<JsonDocSection> = sections
            .iter()
            .map(|s| JsonDocSection::from_doc_section(s, self.include_formatting))
            .collect();
        
        let json_string = if self.pretty {
            serde_json::to_string_pretty(&json_sections)?
        } else {
            serde_json::to_string(&json_sections)?
        };
        
        writeln!(writer, "{}", json_string)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonDocData {
    content: String,
    raw_text: String,
    metadata: JsonDocMetadata,
    sections: Vec<JsonDocSection>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonDocMetadata {
    title: Option<String>,
    author: Option<String>,
    subject: Option<String>,
    created: Option<String>,
    modified: Option<String>,
    word_count: usize,
    paragraph_count: usize,
    page_count: Option<usize>,
    character_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonDocSection {
    section_type: JsonSectionType,
    content: String,
    level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatting: Option<JsonFormatInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "level")]
enum JsonSectionType {
    #[serde(rename = "paragraph")]
    Paragraph,
    #[serde(rename = "heading")]
    Heading(u8),
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "footer")]
    Footer,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "hyperlink")]
    Hyperlink,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonFormatInfo {
    bold: bool,
    italic: bool,
    underline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<String>,
}

impl JsonDocData {
    fn from_doc_data(data: &DocData, include_formatting: bool) -> Self {
        Self {
            content: data.content.clone(),
            raw_text: data.raw_text.clone(),
            metadata: JsonDocMetadata::from_doc_metadata(&data.metadata),
            sections: data
                .sections
                .iter()
                .map(|s| JsonDocSection::from_doc_section(s, include_formatting))
                .collect(),
        }
    }
}

impl JsonDocMetadata {
    fn from_doc_metadata(metadata: &DocMetadata) -> Self {
        Self {
            title: metadata.title.clone(),
            author: metadata.author.clone(),
            subject: metadata.subject.clone(),
            created: metadata.created.clone(),
            modified: metadata.modified.clone(),
            word_count: metadata.word_count,
            paragraph_count: metadata.paragraph_count,
            page_count: metadata.page_count,
            character_count: metadata.character_count,
        }
    }
}

impl JsonDocSection {
    fn from_doc_section(section: &DocSection, include_formatting: bool) -> Self {
        Self {
            section_type: JsonSectionType::from_section_type(&section.section_type),
            content: section.content.clone(),
            level: section.level,
            formatting: if include_formatting {
                section.formatting.as_ref().map(JsonFormatInfo::from_format_info)
            } else {
                None
            },
        }
    }
}

impl JsonSectionType {
    fn from_section_type(section_type: &SectionType) -> Self {
        match section_type {
            SectionType::Paragraph => JsonSectionType::Paragraph,
            SectionType::Heading(level) => JsonSectionType::Heading(*level),
            SectionType::Table => JsonSectionType::Table,
            SectionType::List => JsonSectionType::List,
            SectionType::Image => JsonSectionType::Image,
            SectionType::Footer => JsonSectionType::Footer,
            SectionType::Header => JsonSectionType::Header,
            SectionType::Hyperlink => JsonSectionType::Hyperlink,
        }
    }
}

impl JsonFormatInfo {
    fn from_format_info(format_info: &FormatInfo) -> Self {
        Self {
            bold: format_info.bold,
            italic: format_info.italic,
            underline: format_info.underline,
            font_size: format_info.font_size,
            font_family: format_info.font_family.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{DocMetadata, DocSection, SectionType, FormatInfo};

    #[test]
    fn test_json_output_creation() {
        let _output = JsonOutput::new(false, false);
        let _output_pretty = JsonOutput::new(true, false);
        let _output_with_formatting = JsonOutput::new(false, true);
    }

    #[test]
    fn test_serialize_simple_section() {
        let section = DocSection {
            section_type: SectionType::Paragraph,
            content: "Hello World".to_string(),
            level: None,
            formatting: None,
        };

        let json_section = JsonDocSection::from_doc_section(&section, false);
        let json_string = serde_json::to_string(&json_section).unwrap();
        
        assert!(json_string.contains("\"content\":\"Hello World\""));
        assert!(json_string.contains("\"type\":\"paragraph\""));
    }

    #[test]
    fn test_serialize_heading_section() {
        let section = DocSection {
            section_type: SectionType::Heading(2),
            content: "Chapter Title".to_string(),
            level: Some(2),
            formatting: None,
        };

        let json_section = JsonDocSection::from_doc_section(&section, false);
        let json_string = serde_json::to_string(&json_section).unwrap();
        
        assert!(json_string.contains("\"type\":\"heading\""));
        assert!(json_string.contains("\"level\":2"));
    }

    #[test]
    fn test_serialize_with_formatting() {
        let formatting = FormatInfo {
            bold: true,
            italic: false,
            underline: true,
            font_size: Some(14),
            font_family: Some("Arial".to_string()),
        };

        let section = DocSection {
            section_type: SectionType::Paragraph,
            content: "Formatted text".to_string(),
            level: None,
            formatting: Some(formatting),
        };

        let json_section = JsonDocSection::from_doc_section(&section, true);
        let json_string = serde_json::to_string(&json_section).unwrap();
        
        assert!(json_string.contains("\"bold\":true"));
        assert!(json_string.contains("\"italic\":false"));
        assert!(json_string.contains("\"underline\":true"));
        assert!(json_string.contains("\"font_size\":14"));
        assert!(json_string.contains("\"font_family\":\"Arial\""));
    }

    #[test]
    fn test_serialize_metadata() {
        let metadata = DocMetadata {
            title: Some("Test Document".to_string()),
            author: Some("Test Author".to_string()),
            subject: None,
            created: Some("2023-01-01".to_string()),
            modified: None,
            word_count: 100,
            paragraph_count: 5,
            page_count: Some(2),
            character_count: 500,
        };

        let json_metadata = JsonDocMetadata::from_doc_metadata(&metadata);
        let json_string = serde_json::to_string(&json_metadata).unwrap();
        
        assert!(json_string.contains("\"title\":\"Test Document\""));
        assert!(json_string.contains("\"author\":\"Test Author\""));
        assert!(json_string.contains("\"word_count\":100"));
        assert!(json_string.contains("\"page_count\":2"));
    }

    #[test]
    fn test_pretty_print() {
        let section = DocSection {
            section_type: SectionType::Paragraph,
            content: "Test".to_string(),
            level: None,
            formatting: None,
        };

        let output = JsonOutput::new(true, false);
        let mut buffer = Vec::new();
        output.write_sections(&[section], &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        // Pretty printed JSON should contain indentation
        assert!(result.contains("  "));
    }

    #[test]
    fn test_full_doc_data_json() {
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
            ],
        };

        let output = JsonOutput::new(true, false);
        let mut buffer = Vec::new();
        output.write_doc_data(&doc_data, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        
        // 验证JSON包含关键字段
        assert!(result.contains("\"title\": \"Test Document\""));
        assert!(result.contains("\"author\": \"Test Author\""));
        assert!(result.contains("\"word_count\": 6"));
        assert!(result.contains("\"type\": \"heading\""));
        assert!(result.contains("\"type\": \"paragraph\""));
        
        // 验证JSON是有效的
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["metadata"]["title"].is_string());
        assert!(parsed["sections"].is_array());
    }
}