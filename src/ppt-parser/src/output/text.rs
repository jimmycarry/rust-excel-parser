use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{PptData, Slide, ListType};
use std::io::Write;

pub struct TextOutput;

impl TextOutput {
    pub fn new() -> Self {
        Self
    }
}

impl OutputWriter for TextOutput {
    fn write_ppt_data<W: Write>(&self, data: &PptData, writer: &mut W) -> Result<()> {
        // Write presentation metadata as header
        if let Some(title) = &data.metadata.title {
            writeln!(writer, "Presentation: {}", title)?;
        } else {
            writeln!(writer, "Presentation")?;
        }
        
        if let Some(author) = &data.metadata.author {
            writeln!(writer, "Author: {}", author)?;
        }
        
        writeln!(writer, "Slides: {}", data.slide_count)?;
        
        if let Some(creation_date) = &data.metadata.creation_date {
            writeln!(writer, "Created: {}", creation_date.format("%Y-%m-%d %H:%M:%S UTC"))?;
        }
        
        writeln!(writer)?; // Empty line
        writeln!(writer, "{}", "=".repeat(80))?;
        writeln!(writer)?;

        // Write each slide
        for (i, slide) in data.slides.iter().enumerate() {
            if i > 0 {
                writeln!(writer)?; // Empty line between slides
                writeln!(writer, "{}", "-".repeat(80))?;
                writeln!(writer)?;
            }
            self.write_slide(slide, writer)?;
        }

        Ok(())
    }

    fn write_slide<W: Write>(&self, slide: &Slide, writer: &mut W) -> Result<()> {
        // Write slide header
        writeln!(writer, "SLIDE {}", slide.number)?;
        writeln!(writer)?;

        // Write title if present
        if let Some(title) = &slide.title {
            writeln!(writer, "Title: {}", title)?;
            writeln!(writer)?;
        }

        // Write content
        if !slide.content.is_empty() {
            writeln!(writer, "Content:")?;
            for content_item in &slide.content {
                writeln!(writer, "  {}", content_item)?;
            }
            writeln!(writer)?;
        }

        // Write lists
        if !slide.lists.is_empty() {
            writeln!(writer, "Lists:")?;
            for list in &slide.lists {
                match list.list_type {
                    ListType::Ordered => {
                        for (i, item) in list.items.iter().enumerate() {
                            writeln!(writer, "  {}. {}", i + 1, item)?;
                        }
                    }
                    ListType::Unordered => {
                        for item in &list.items {
                            writeln!(writer, "  • {}", item)?;
                        }
                    }
                }
            }
            writeln!(writer)?;
        }

        // Write tables
        if !slide.tables.is_empty() {
            writeln!(writer, "Tables:")?;
            for table in &slide.tables {
                if let Some(headers) = &table.headers {
                    // Write headers
                    let header_line = headers.join(" | ");
                    writeln!(writer, "  {}", header_line)?;
                    writeln!(writer, "  {}", "-".repeat(header_line.len()))?;
                    
                    // Write data rows (skip first row if it's headers)
                    for row in &table.rows[1..] {
                        writeln!(writer, "  {}", row.join(" | "))?;
                    }
                } else {
                    // Write all rows
                    for row in &table.rows {
                        writeln!(writer, "  {}", row.join(" | "))?;
                    }
                }
                writeln!(writer)?;
            }
        }

        // Write notes if present
        if let Some(notes) = &slide.notes {
            writeln!(writer, "Notes:")?;
            writeln!(writer, "  {}", notes)?;
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
    fn test_text_slide_output() {
        let slide = Slide {
            number: 1,
            title: Some("Test Slide".to_string()),
            content: vec!["First point".to_string(), "Second point".to_string()],
            notes: Some("Important notes".to_string()),
            tables: vec![],
            lists: vec![],
        };

        let output = TextOutput::new();
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("SLIDE 1"));
        assert!(result.contains("Title: Test Slide"));
        assert!(result.contains("First point"));
        assert!(result.contains("Second point"));
        assert!(result.contains("Notes:"));
        assert!(result.contains("Important notes"));
    }

    #[test]
    fn test_text_slide_with_list() {
        let list = List {
            slide_number: 1,
            list_type: ListType::Unordered,
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
        };

        let slide = Slide {
            number: 1,
            title: None,
            content: vec![],
            notes: None,
            tables: vec![],
            lists: vec![list],
        };

        let output = TextOutput::new();
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Lists:"));
        assert!(result.contains("• Item 1"));
        assert!(result.contains("• Item 2"));
    }

    #[test]
    fn test_text_slide_with_table() {
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

        let output = TextOutput::new();
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Tables:"));
        assert!(result.contains("Name | Age"));
        assert!(result.contains("John | 30"));
        assert!(result.contains("Jane | 25"));
    }

    #[test]
    fn test_text_ppt_data_output() {
        let metadata = PptMetadata {
            title: Some("Test Presentation".to_string()),
            author: Some("Test Author".to_string()),
            subject: None,
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

        let output = TextOutput::new();
        let mut buffer = Vec::new();
        output.write_ppt_data(&ppt_data, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Presentation: Test Presentation"));
        assert!(result.contains("Author: Test Author"));
        assert!(result.contains("Slides: 1"));
        assert!(result.contains("SLIDE 1"));
    }
}