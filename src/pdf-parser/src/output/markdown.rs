use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{PdfData, Page, Table};
use std::io::Write;

pub struct MarkdownOutput {
    include_metadata: bool,
}

impl MarkdownOutput {
    pub fn new(include_metadata: bool) -> Self {
        Self { include_metadata }
    }
}

impl OutputWriter for MarkdownOutput {
    fn write_pdf_data<W: Write>(&self, data: &PdfData, writer: &mut W) -> Result<()> {
        // Write metadata if requested
        if self.include_metadata {
            writeln!(writer, "# PDF Document\n")?;
            
            if let Some(title) = &data.metadata.title {
                writeln!(writer, "**Title:** {}\n", title)?;
            }
            if let Some(author) = &data.metadata.author {
                writeln!(writer, "**Author:** {}\n", author)?;
            }
            if let Some(subject) = &data.metadata.subject {
                writeln!(writer, "**Subject:** {}\n", subject)?;
            }
            if let Some(creator) = &data.metadata.creator {
                writeln!(writer, "**Creator:** {}\n", creator)?;
            }
            if let Some(producer) = &data.metadata.producer {
                writeln!(writer, "**Producer:** {}\n", producer)?;
            }
            if let Some(creation_date) = &data.metadata.creation_date {
                writeln!(writer, "**Created:** {}\n", creation_date.format("%Y-%m-%d %H:%M:%S UTC"))?;
            }
            if let Some(modification_date) = &data.metadata.modification_date {
                writeln!(writer, "**Modified:** {}\n", modification_date.format("%Y-%m-%d %H:%M:%S UTC"))?;
            }
            
            writeln!(writer, "**Pages:** {}", data.metadata.page_count)?;
            writeln!(writer, "**File Size:** {} bytes\n", data.metadata.file_size)?;
            writeln!(writer, "---\n")?;
        }

        // Write pages
        for page in &data.pages {
            writeln!(writer, "## Page {}\n", page.number)?;
            
            // Write page text with proper markdown formatting
            let formatted_text = self.format_text_content(&page.text);
            writeln!(writer, "{}\n", formatted_text)?;
            
            // Write tables for this page
            if !page.tables.is_empty() {
                writeln!(writer, "### Tables\n")?;
                for (i, table) in page.tables.iter().enumerate() {
                    writeln!(writer, "#### Table {}\n", i + 1)?;
                    self.write_table_markdown(table, writer)?;
                    writeln!(writer)?;
                }
            }
        }

        Ok(())
    }

    fn write_page<W: Write>(&self, page: &Page, writer: &mut W) -> Result<()> {
        writeln!(writer, "# Page {}\n", page.number)?;
        
        let formatted_text = self.format_text_content(&page.text);
        writeln!(writer, "{}", formatted_text)?;
        
        if !page.tables.is_empty() {
            writeln!(writer, "\n## Tables\n")?;
            for (i, table) in page.tables.iter().enumerate() {
                writeln!(writer, "### Table {}\n", i + 1)?;
                self.write_table_markdown(table, writer)?;
                writeln!(writer)?;
            }
        }
        
        Ok(())
    }

    fn write_tables<W: Write>(&self, tables: &[Table], writer: &mut W) -> Result<()> {
        writeln!(writer, "# Tables\n")?;
        
        for (i, table) in tables.iter().enumerate() {
            writeln!(writer, "## Table {} (Page {})\n", i + 1, table.page)?;
            self.write_table_markdown(table, writer)?;
            writeln!(writer)?;
        }
        
        Ok(())
    }
}

impl MarkdownOutput {
    fn format_text_content(&self, text: &str) -> String {
        // Basic text formatting for markdown
        let mut formatted = text.to_string();
        
        // Convert multiple newlines to proper paragraph breaks
        formatted = formatted.replace("\n\n\n", "\n\n");
        
        // Escape markdown special characters in regular text
        formatted = formatted.replace("*", "\\*");
        formatted = formatted.replace("_", "\\_");
        formatted = formatted.replace("#", "\\#");
        formatted = formatted.replace("`", "\\`");
        
        formatted
    }

    fn write_table_markdown<W: Write>(&self, table: &Table, writer: &mut W) -> Result<()> {
        if table.data.is_empty() {
            return Ok(());
        }

        // Determine if we have headers
        let (headers, data_rows) = if let Some(headers) = &table.headers {
            (headers.clone(), &table.data[..])
        } else if !table.data.is_empty() {
            // Use first row as headers
            (table.data[0].clone(), &table.data[1..])
        } else {
            return Ok(());
        };

        // Write headers
        write!(writer, "|")?;
        for header in &headers {
            write!(writer, " {} |", self.escape_markdown_table_cell(header))?;
        }
        writeln!(writer)?;

        // Write separator
        write!(writer, "|")?;
        for _ in &headers {
            write!(writer, " --- |")?;
        }
        writeln!(writer)?;

        // Write data rows
        for row in data_rows {
            write!(writer, "|")?;
            for (i, cell) in row.iter().enumerate() {
                if i < headers.len() {
                    write!(writer, " {} |", self.escape_markdown_table_cell(cell))?;
                }
            }
            // Fill empty cells if row is shorter than headers
            for _ in row.len()..headers.len() {
                write!(writer, "  |")?;
            }
            writeln!(writer)?;
        }

        Ok(())
    }

    fn escape_markdown_table_cell(&self, cell: &str) -> String {
        // Escape special characters in table cells
        cell.replace("|", "\\|")
            .replace("\n", " ")
            .replace("\r", "")
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::PdfMetadata;

    #[test]
    fn test_markdown_page_output() {
        let page = Page {
            number: 1,
            text: "Sample text content".to_string(),
            tables: vec![],
        };

        let output = MarkdownOutput::new(false);
        let mut buffer = Vec::new();
        output.write_page(&page, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("# Page 1"));
        assert!(result.contains("Sample text content"));
    }

    #[test]
    fn test_markdown_table_output() {
        let table = Table {
            page: 1,
            data: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "30".to_string()],
                vec!["Jane".to_string(), "25".to_string()],
            ],
            headers: Some(vec!["Name".to_string(), "Age".to_string()]),
        };

        let output = MarkdownOutput::new(false);
        let mut buffer = Vec::new();
        output.write_tables(&[table], &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("| Name | Age |"));
        assert!(result.contains("| --- | --- |"));
        assert!(result.contains("| John | 30 |"));
    }

    #[test]
    fn test_escape_markdown_special_chars() {
        let output = MarkdownOutput::new(false);
        let result = output.format_text_content("Text with *asterisk* and _underscore_");
        assert!(result.contains("\\*asterisk\\*"));
        assert!(result.contains("\\_underscore\\_"));
    }
}