use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{PptData, Slide, ListType};
use std::io::Write;

pub struct MarkdownOutput {
    include_metadata: bool,
    include_slide_numbers: bool,
}

impl MarkdownOutput {
    pub fn new(include_metadata: bool, include_slide_numbers: bool) -> Self {
        Self {
            include_metadata,
            include_slide_numbers,
        }
    }
}

impl OutputWriter for MarkdownOutput {
    fn write_ppt_data<W: Write>(&self, data: &PptData, writer: &mut W) -> Result<()> {
        // Write YAML frontmatter if metadata is requested
        if self.include_metadata {
            writeln!(writer, "---")?;
            if let Some(title) = &data.metadata.title {
                writeln!(writer, "title: \"{}\"", title.replace('"', "\\\""))?;
            }
            if let Some(author) = &data.metadata.author {
                writeln!(writer, "author: \"{}\"", author.replace('"', "\\\""))?;
            }
            if let Some(subject) = &data.metadata.subject {
                writeln!(writer, "subject: \"{}\"", subject.replace('"', "\\\""))?;
            }
            if let Some(creation_date) = &data.metadata.creation_date {
                writeln!(writer, "date: \"{}\"", creation_date.format("%Y-%m-%d"))?;
            }
            writeln!(writer, "slide_count: {}", data.slide_count)?;
            writeln!(writer, "file_size: {}", data.metadata.file_size)?;
            writeln!(writer, "---")?;
            writeln!(writer)?;
        }

        // Write presentation title as main header
        if let Some(title) = &data.metadata.title {
            writeln!(writer, "# {}", title)?;
            writeln!(writer)?;
        }

        // Write metadata summary
        if self.include_metadata {
            writeln!(writer, "## Presentation Information")?;
            writeln!(writer)?;
            if let Some(author) = &data.metadata.author {
                writeln!(writer, "**Author:** {}", author)?;
            }
            if let Some(creation_date) = &data.metadata.creation_date {
                writeln!(writer, "**Created:** {}", creation_date.format("%Y-%m-%d %H:%M:%S UTC"))?;
            }
            writeln!(writer, "**Slides:** {}", data.slide_count)?;
            writeln!(writer)?;
            writeln!(writer, "---")?;
            writeln!(writer)?;
        }

        // Write each slide
        for slide in &data.slides {
            self.write_slide(slide, writer)?;
            writeln!(writer)?; // Empty line between slides
        }

        Ok(())
    }

    fn write_slide<W: Write>(&self, slide: &Slide, writer: &mut W) -> Result<()> {
        // Write slide header
        if self.include_slide_numbers {
            if let Some(title) = &slide.title {
                writeln!(writer, "## Slide {}: {}", slide.number, title)?;
            } else {
                writeln!(writer, "## Slide {}", slide.number)?;
            }
        } else if let Some(title) = &slide.title {
            writeln!(writer, "## {}", title)?;
        } else {
            writeln!(writer, "## Untitled Slide")?;
        }
        writeln!(writer)?;

        // Write content
        if !slide.content.is_empty() {
            for content_item in &slide.content {
                writeln!(writer, "{}", content_item)?;
                writeln!(writer)?;
            }
        }

        // Write lists
        for list in &slide.lists {
            match list.list_type {
                ListType::Ordered => {
                    for (i, item) in list.items.iter().enumerate() {
                        writeln!(writer, "{}. {}", i + 1, item)?;
                    }
                }
                ListType::Unordered => {
                    for item in &list.items {
                        writeln!(writer, "- {}", item)?;
                    }
                }
            }
            writeln!(writer)?;
        }

        // Write tables
        for table in &slide.tables {
            if !table.rows.is_empty() {
                // Write table headers
                if let Some(headers) = &table.headers {
                    writeln!(writer, "| {} |", headers.join(" | "))?;
                    writeln!(writer, "|{}|", vec!["---"; headers.len()].join("|"))?;
                    
                    // Write data rows (skip first row if it's headers)
                    for row in &table.rows[1..] {
                        writeln!(writer, "| {} |", row.join(" | "))?;
                    }
                } else {
                    // Treat first row as headers
                    if !table.rows.is_empty() {
                        writeln!(writer, "| {} |", table.rows[0].join(" | "))?;
                        writeln!(writer, "|{}|", vec!["---"; table.rows[0].len()].join("|"))?;
                        
                        for row in &table.rows[1..] {
                            writeln!(writer, "| {} |", row.join(" | "))?;
                        }
                    }
                }
                writeln!(writer)?;
            }
        }

        // Write notes as blockquote
        if let Some(notes) = &slide.notes {
            writeln!(writer, "> **Speaker Notes:** {}", notes)?;
            writeln!(writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{List, ListType, PptMetadata, Table};
    use chrono::Utc;

    #[test]
    fn test_markdown_slide_output() {
        let slide = Slide {
            number: 1,
            title: Some("Test Slide".to_string()),
            content: vec!["First point".to_string(), "Second point".to_string()],
            notes: Some("Important notes".to_string()),
            tables: vec![],
            lists: vec![],
        };

        let output = MarkdownOutput::new(false, true);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("## Slide 1: Test Slide"));
        assert!(result.contains("First point"));
        assert!(result.contains("Second point"));
        assert!(result.contains("> **Speaker Notes:** Important notes"));
    }

    #[test]
    fn test_markdown_slide_with_list() {
        let list = List {
            slide_number: 1,
            list_type: ListType::Unordered,
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
        };

        let slide = Slide {
            number: 1,
            title: Some("List Slide".to_string()),
            content: vec![],
            notes: None,
            tables: vec![],
            lists: vec![list],
        };

        let output = MarkdownOutput::new(false, true);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("- Item 1"));
        assert!(result.contains("- Item 2"));
    }

    #[test]
    fn test_markdown_slide_with_ordered_list() {
        let list = List {
            slide_number: 1,
            list_type: ListType::Ordered,
            items: vec!["First".to_string(), "Second".to_string()],
        };

        let slide = Slide {
            number: 1,
            title: None,
            content: vec![],
            notes: None,
            tables: vec![],
            lists: vec![list],
        };

        let output = MarkdownOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("1. First"));
        assert!(result.contains("2. Second"));
    }

    #[test]
    fn test_markdown_slide_with_table() {
        let table = Table {
            slide_number: 1,
            rows: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "30".to_string()],
                vec!["Jane".to_string(), "25".to_string()],
            ],
            headers: Some(vec!["Name".to_string(), "Age".to_string()]),
        };

        let slide = Slide {
            number: 1,
            title: None,
            content: vec![],
            notes: None,
            tables: vec![table],
            lists: vec![],
        };

        let output = MarkdownOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("| Name | Age |"));
        assert!(result.contains("|---|---|"));
        assert!(result.contains("| John | 30 |"));
        assert!(result.contains("| Jane | 25 |"));
    }

    #[test]
    fn test_markdown_ppt_data_with_metadata() {
        let metadata = PptMetadata {
            title: Some("Test Presentation".to_string()),
            author: Some("Test Author".to_string()),
            subject: Some("Test Subject".to_string()),
            creator: None,
            creation_date: Some(Utc::now()),
            modification_date: None,
            slide_count: 1,
            file_size: 1024,
            application: None,
        };

        let slide = Slide {
            number: 1,
            title: Some("Slide 1".to_string()),
            content: vec!["Content".to_string()],
            notes: None,
            tables: vec![],
            lists: vec![],
        };

        let ppt_data = PptData {
            slides: vec![slide],
            metadata,
            slide_count: 1,
        };

        let output = MarkdownOutput::new(true, true);
        let mut buffer = Vec::new();
        output.write_ppt_data(&ppt_data, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("---"));
        assert!(result.contains("title: \"Test Presentation\""));
        assert!(result.contains("author: \"Test Author\""));
        assert!(result.contains("subject: \"Test Subject\""));
        assert!(result.contains("slide_count: 1"));
        assert!(result.contains("# Test Presentation"));
        assert!(result.contains("## Presentation Information"));
        assert!(result.contains("**Author:** Test Author"));
        assert!(result.contains("**Slides:** 1"));
        assert!(result.contains("## Slide 1: Slide 1"));
    }

    #[test]
    fn test_markdown_escape_quotes() {
        let metadata = PptMetadata {
            title: Some("Test \"Quoted\" Title".to_string()),
            author: Some("Author with \"quotes\"".to_string()),
            subject: None,
            creator: None,
            creation_date: None,
            modification_date: None,
            slide_count: 1,
            file_size: 1024,
            application: None,
        };

        let slide = Slide {
            number: 1,
            title: None,
            content: vec![],
            notes: None,
            tables: vec![],
            lists: vec![],
        };

        let ppt_data = PptData {
            slides: vec![slide],
            metadata,
            slide_count: 1,
        };

        let output = MarkdownOutput::new(true, false);
        let mut buffer = Vec::new();
        output.write_ppt_data(&ppt_data, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("title: \"Test \\\"Quoted\\\" Title\""));
        assert!(result.contains("author: \"Author with \\\"quotes\\\"\""));
    }
}