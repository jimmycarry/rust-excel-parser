use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{PdfData, Page, Table};
use serde_json;
use std::io::Write;

pub struct JsonOutput {
    pretty: bool,
    include_metadata: bool,
}

impl JsonOutput {
    pub fn new(pretty: bool, include_metadata: bool) -> Self {
        Self {
            pretty,
            include_metadata,
        }
    }
}

impl OutputWriter for JsonOutput {
    fn write_pdf_data<W: Write>(&self, data: &PdfData, writer: &mut W) -> Result<()> {
        let json_data = if self.include_metadata {
            serde_json::json!({
                "metadata": data.metadata,
                "pages": data.pages,
                "tables": data.tables
            })
        } else {
            serde_json::json!({
                "pages": data.pages,
                "tables": data.tables
            })
        };

        if self.pretty {
            serde_json::to_writer_pretty(writer, &json_data)?;
        } else {
            serde_json::to_writer(writer, &json_data)?;
        }
        Ok(())
    }

    fn write_page<W: Write>(&self, page: &Page, writer: &mut W) -> Result<()> {
        if self.pretty {
            serde_json::to_writer_pretty(writer, page)?;
        } else {
            serde_json::to_writer(writer, page)?;
        }
        Ok(())
    }

    fn write_tables<W: Write>(&self, tables: &[Table], writer: &mut W) -> Result<()> {
        if self.pretty {
            serde_json::to_writer_pretty(writer, tables)?;
        } else {
            serde_json::to_writer(writer, tables)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::PdfMetadata;

    #[test]
    fn test_json_page_output() {
        let page = Page {
            number: 1,
            text: "Sample text content".to_string(),
            tables: vec![],
        };

        let output = JsonOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_page(&page, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert_eq!(parsed["number"], 1);
        assert_eq!(parsed["text"], "Sample text content");
    }

    #[test]
    fn test_json_pretty_output() {
        let page = Page {
            number: 1,
            text: "Sample text".to_string(),
            tables: vec![],
        };

        let output = JsonOutput::new(true, false);
        let mut buffer = Vec::new();
        output.write_page(&page, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        // Pretty printed JSON should have newlines
        assert!(result.contains('\n'));
    }

    #[test]
    fn test_json_tables_output() {
        let table = Table {
            page: 1,
            data: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "30".to_string()],
            ],
            headers: Some(vec!["Name".to_string(), "Age".to_string()]),
        };

        let output = JsonOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_tables(&[table], &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert_eq!(parsed[0]["page"], 1);
        assert_eq!(parsed[0]["data"][0][0], "Name");
    }
}