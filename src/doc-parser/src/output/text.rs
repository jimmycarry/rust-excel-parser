use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{DocData, DocSection, SectionType};
use std::io::Write;

pub struct TextOutput {
    preserve_formatting: bool,
    include_metadata: bool,
    line_numbers: bool,
}

impl TextOutput {
    pub fn new(preserve_formatting: bool, include_metadata: bool, line_numbers: bool) -> Self {
        Self {
            preserve_formatting,
            include_metadata,
            line_numbers,
        }
    }
}

impl OutputWriter for TextOutput {
    fn write_doc_data<W: Write>(&self, data: &DocData, writer: &mut W) -> Result<()> {
        if self.include_metadata {
            write_metadata(writer, &data.metadata)?;
            writeln!(writer)?;
        }

        if self.preserve_formatting {
            self.write_sections(&data.sections, writer)?;
        } else {
            if self.line_numbers {
                write_text_with_line_numbers(writer, &data.raw_text)?;
            } else {
                writeln!(writer, "{}", data.raw_text)?;
            }
        }

        Ok(())
    }

    fn write_sections<W: Write>(&self, sections: &[DocSection], writer: &mut W) -> Result<()> {
        let mut line_number = 1;

        for section in sections {
            let formatted_content = if self.preserve_formatting {
                format_section_content(section)
            } else {
                section.content.clone()
            };

            if self.line_numbers {
                for line in formatted_content.lines() {
                    writeln!(writer, "{:4}: {}", line_number, line)?;
                    line_number += 1;
                }
            } else {
                writeln!(writer, "{}", formatted_content)?;
            }
        }

        Ok(())
    }
}

fn write_metadata<W: Write>(writer: &mut W, metadata: &crate::parser::DocMetadata) -> Result<()> {
    writeln!(writer, "=== 文档元数据 ===")?;
    
    if let Some(title) = &metadata.title {
        writeln!(writer, "标题: {}", title)?;
    }
    if let Some(author) = &metadata.author {
        writeln!(writer, "作者: {}", author)?;
    }
    if let Some(subject) = &metadata.subject {
        writeln!(writer, "主题: {}", subject)?;
    }
    if let Some(created) = &metadata.created {
        writeln!(writer, "创建时间: {}", created)?;
    }
    if let Some(modified) = &metadata.modified {
        writeln!(writer, "修改时间: {}", modified)?;
    }
    
    writeln!(writer, "段落数: {}", metadata.paragraph_count)?;
    writeln!(writer, "字数: {}", metadata.word_count)?;
    writeln!(writer, "字符数: {}", metadata.character_count)?;
    
    if let Some(page_count) = metadata.page_count {
        writeln!(writer, "页数: {}", page_count)?;
    }
    
    writeln!(writer, "\n=== 文档内容 ===")?;
    
    Ok(())
}

fn write_text_with_line_numbers<W: Write>(writer: &mut W, text: &str) -> Result<()> {
    for (line_number, line) in text.lines().enumerate() {
        writeln!(writer, "{:4}: {}", line_number + 1, line)?;
    }
    Ok(())
}

fn format_section_content(section: &DocSection) -> String {
    match &section.section_type {
        SectionType::Heading(level) => {
            format!("{} {}", "#".repeat(*level as usize), section.content)
        }
        SectionType::List => {
            format!("• {}", section.content)
        }
        SectionType::Table => {
            format!("[表格] {}", section.content)
        }
        SectionType::Image => {
            format!("[图片] {}", section.content)
        }
        SectionType::Hyperlink => {
            format!("[链接] {}", section.content)
        }
        SectionType::Header => {
            format!("[页眉] {}", section.content)
        }
        SectionType::Footer => {
            format!("[页脚] {}", section.content)
        }
        SectionType::Paragraph => section.content.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{DocMetadata, DocSection, SectionType};

    #[test]
    fn test_text_output_creation() {
        let _output = TextOutput::new(false, false, false);
        let _output_with_metadata = TextOutput::new(false, true, false);
        let _output_with_formatting = TextOutput::new(true, false, false);
        let _output_with_line_numbers = TextOutput::new(false, false, true);
    }

    #[test]
    fn test_write_simple_sections() {
        let sections = vec![
            DocSection {
                section_type: SectionType::Paragraph,
                content: "Hello World".to_string(),
                level: None,
                formatting: None,
            },
            DocSection {
                section_type: SectionType::Heading(1),
                content: "Title".to_string(),
                level: Some(1),
                formatting: None,
            },
        ];

        let output = TextOutput::new(true, false, false);
        let mut buffer = Vec::new();
        output.write_sections(&sections, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Hello World"));
        assert!(result.contains("# Title"));
    }

    #[test]
    fn test_write_with_line_numbers() {
        let sections = vec![
            DocSection {
                section_type: SectionType::Paragraph,
                content: "Line 1".to_string(),
                level: None,
                formatting: None,
            },
            DocSection {
                section_type: SectionType::Paragraph,
                content: "Line 2".to_string(),
                level: None,
                formatting: None,
            },
        ];

        let output = TextOutput::new(false, false, true);
        let mut buffer = Vec::new();
        output.write_sections(&sections, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("   1: Line 1"));
        assert!(result.contains("   2: Line 2"));
    }
}