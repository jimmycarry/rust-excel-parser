use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{PdfData, Page, Table};
use std::io::Write;

pub struct TextOutput;

impl TextOutput {
    pub fn new() -> Self {
        Self
    }
}

impl OutputWriter for TextOutput {
    fn write_pdf_data<W: Write>(&self, data: &PdfData, writer: &mut W) -> Result<()> {
        for (i, page) in data.pages.iter().enumerate() {
            if i > 0 {
                writeln!(writer, "\n--- Page {} ---\n", page.number)?;
            }
            write!(writer, "{}", page.text)?;
        }
        Ok(())
    }

    fn write_page<W: Write>(&self, page: &Page, writer: &mut W) -> Result<()> {
        write!(writer, "{}", page.text)?;
        Ok(())
    }

    fn write_tables<W: Write>(&self, tables: &[Table], writer: &mut W) -> Result<()> {
        for (i, table) in tables.iter().enumerate() {
            if i > 0 {
                writeln!(writer)?;
            }
            
            writeln!(writer, "--- Table {} (Page {}) ---", i + 1, table.page)?;
            
            if let Some(headers) = &table.headers {
                writeln!(writer, "{}", headers.join("\t"))?;
                writeln!(writer, "{}", "-".repeat(headers.join("\t").len()))?;
            }
            
            for row in &table.data {
                writeln!(writer, "{}", row.join("\t"))?;
            }
        }
        Ok(())
    }
}

impl Default for TextOutput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::PdfMetadata;

    #[test]
    fn test_text_output() {
        let page = Page {
            number: 1,
            text: "Sample text content".to_string(),
            tables: vec![],
        };

        let output = TextOutput::new();
        let mut buffer = Vec::new();
        output.write_page(&page, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "Sample text content");
    }

    #[test]
    fn test_table_output() {
        let table = Table {
            page: 1,
            data: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "30".to_string()],
                vec!["Jane".to_string(), "25".to_string()],
            ],
            headers: Some(vec!["Name".to_string(), "Age".to_string()]),
        };

        let output = TextOutput::new();
        let mut buffer = Vec::new();
        output.write_tables(&[table], &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Name\tAge"));
        assert!(result.contains("John\t30"));
    }
}