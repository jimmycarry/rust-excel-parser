use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{PdfData, Page, Table};
use csv::WriterBuilder;
use std::io::Write;

pub struct CsvOutput {
    delimiter: u8,
    quote_char: u8,
    has_headers: bool,
}

impl CsvOutput {
    pub fn new(delimiter: u8, quote_char: u8, has_headers: bool) -> Self {
        Self {
            delimiter,
            quote_char,
            has_headers,
        }
    }
}

impl OutputWriter for CsvOutput {
    fn write_pdf_data<W: Write>(&self, data: &PdfData, writer: &mut W) -> Result<()> {
        // For PDF data, we extract all tables and output them as CSV
        if data.tables.is_empty() {
            // If no tables, create a simple CSV with page content
            let mut csv_writer = WriterBuilder::new()
                .delimiter(self.delimiter)
                .quote(self.quote_char)
                .from_writer(writer);

            if self.has_headers {
                csv_writer.write_record(&["Page", "Content"])?;
            }

            for page in &data.pages {
                // Clean up text for CSV (remove newlines, etc.)
                let cleaned_text = page.text.replace('\n', " ").replace('\r', "");
                csv_writer.write_record(&[&page.number.to_string(), &cleaned_text])?;
            }
        } else {
            // Output all tables
            self.write_tables(&data.tables, writer)?;
        }

        Ok(())
    }

    fn write_page<W: Write>(&self, page: &Page, writer: &mut W) -> Result<()> {
        if page.tables.is_empty() {
            // No tables in page, output page content
            let mut csv_writer = WriterBuilder::new()
                .delimiter(self.delimiter)
                .quote(self.quote_char)
                .from_writer(writer);

            if self.has_headers {
                csv_writer.write_record(&["Content"])?;
            }

            let cleaned_text = page.text.replace('\n', " ").replace('\r', "");
            csv_writer.write_record(&[&cleaned_text])?;
        } else {
            // Output tables from this page
            self.write_tables(&page.tables, writer)?;
        }

        Ok(())
    }

    fn write_tables<W: Write>(&self, tables: &[Table], writer: &mut W) -> Result<()> {
        if tables.is_empty() {
            return Ok(());
        }

        let mut csv_writer = WriterBuilder::new()
            .delimiter(self.delimiter)
            .quote(self.quote_char)
            .from_writer(writer);

        let mut first_table = true;

        for table in tables {
            if table.data.is_empty() {
                continue;
            }

            // Normalize table data to ensure consistent column counts
            let normalized_table = self.normalize_table_data(table);
            if normalized_table.is_empty() {
                continue;
            }

            if !first_table {
                // Add separator between tables (as a comment row)
                let separator_row = vec![format!("--- Table from Page {} ---", table.page)];
                // Pad separator to match table width
                let mut padded_separator = separator_row;
                while padded_separator.len() < normalized_table[0].len() {
                    padded_separator.push(String::new());
                }
                csv_writer.write_record(&padded_separator)?;
            }
            first_table = false;

            // Write headers if available and requested
            if self.has_headers {
                if let Some(headers) = &table.headers {
                    let mut normalized_headers = headers.clone();
                    // Ensure headers match table width
                    while normalized_headers.len() < normalized_table[0].len() {
                        normalized_headers.push(format!("Column_{}", normalized_headers.len() + 1));
                    }
                    normalized_headers.truncate(normalized_table[0].len());
                    csv_writer.write_record(&normalized_headers)?;
                } else if !normalized_table.is_empty() {
                    // Use first row as headers
                    csv_writer.write_record(&normalized_table[0])?;
                    // Write remaining data
                    for row in &normalized_table[1..] {
                        csv_writer.write_record(row)?;
                    }
                    continue;
                }
            }

            // Write data rows
            for row in &normalized_table {
                csv_writer.write_record(row)?;
            }
        }

        Ok(())
    }

}

impl CsvOutput {
    fn normalize_table_data(&self, table: &Table) -> Vec<Vec<String>> {
        if table.data.is_empty() {
            return Vec::new();
        }

        // Find the maximum column count
        let max_columns = table.data.iter()
            .map(|row| row.len())
            .max()
            .unwrap_or(0);

        if max_columns == 0 {
            return Vec::new();
        }

        // Normalize all rows to have the same number of columns
        table.data.iter()
            .map(|row| {
                let mut normalized_row = row.clone();
                
                // Pad with empty strings if row is too short
                while normalized_row.len() < max_columns {
                    normalized_row.push(String::new());
                }
                
                // Truncate if row is too long
                normalized_row.truncate(max_columns);
                
                // Clean up cell content
                normalized_row.iter()
                    .map(|cell| {
                        cell.trim()
                            .replace('\n', " ")
                            .replace('\r', "")
                            .replace('\t', " ")
                    })
                    .collect()
            })
            .filter(|row: &Vec<String>| {
                // Filter out rows that are completely empty
                row.iter().any(|cell| !cell.trim().is_empty())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::PdfMetadata;

    #[test]
    fn test_csv_table_output() {
        let table = Table {
            page: 1,
            data: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "30".to_string()],
                vec!["Jane".to_string(), "25".to_string()],
            ],
            headers: Some(vec!["Name".to_string(), "Age".to_string()]),
        };

        let output = CsvOutput::new(b',', b'"', true);
        let mut buffer = Vec::new();
        output.write_tables(&[table], &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Name,Age"));
        assert!(result.contains("John,30"));
        assert!(result.contains("Jane,25"));
    }

    #[test]
    fn test_csv_no_headers() {
        let table = Table {
            page: 1,
            data: vec![
                vec!["John".to_string(), "30".to_string()],
                vec!["Jane".to_string(), "25".to_string()],
            ],
            headers: None,
        };

        let output = CsvOutput::new(b',', b'"', false);
        let mut buffer = Vec::new();
        output.write_tables(&[table], &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("John,30"));
        assert!(result.contains("Jane,25"));
        // Should not contain header row
        assert!(!result.starts_with("Name,Age"));
    }

    #[test]
    fn test_csv_custom_delimiter() {
        let table = Table {
            page: 1,
            data: vec![
                vec!["John".to_string(), "30".to_string()],
            ],
            headers: None,
        };

        let output = CsvOutput::new(b'\t', b'"', false);
        let mut buffer = Vec::new();
        output.write_tables(&[table], &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("John\t30"));
    }

    #[test]
    fn test_csv_page_output_no_tables() {
        let page = Page {
            number: 1,
            text: "Simple text content".to_string(),
            tables: vec![],
        };

        let output = CsvOutput::new(b',', b'"', true);
        let mut buffer = Vec::new();
        output.write_page(&page, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Content"));
        assert!(result.contains("Simple text content"));
    }
}