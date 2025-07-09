use std::io::Write;
use csv::WriterBuilder;
use crate::error::Result;
use crate::parser::{ExcelData, Sheet};
use super::OutputWriter;

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

    fn create_writer<W: Write>(&self, writer: W) -> csv::Writer<W> {
        WriterBuilder::new()
            .delimiter(self.delimiter)
            .quote(self.quote_char)
            .has_headers(self.has_headers)
            .from_writer(writer)
    }
}

impl OutputWriter for CsvOutput {
    fn write_excel_data<W: Write>(&self, data: &ExcelData, writer: &mut W) -> Result<()> {
        // For multiple sheets, we'll write them separated by empty lines
        // and include sheet names as comments
        let mut first_sheet = true;
        
        for sheet in &data.sheets {
            if !first_sheet {
                // Add separator between sheets
                writeln!(writer)?;
                writeln!(writer, "# Sheet: {}", sheet.name)?;
            } else if data.sheets.len() > 1 {
                // Add sheet name for first sheet if there are multiple sheets
                writeln!(writer, "# Sheet: {}", sheet.name)?;
            }
            
            self.write_sheet(sheet, writer)?;
            first_sheet = false;
        }
        
        Ok(())
    }

    fn write_sheet<W: Write>(&self, sheet: &Sheet, writer: &mut W) -> Result<()> {
        if sheet.data.is_empty() {
            return Ok(());
        }

        let mut csv_writer = self.create_writer(writer);
        
        for row in &sheet.data {
            // Convert all cell values to strings and write as CSV record
            let string_row: Vec<&str> = row.iter().map(|s| s.as_str()).collect();
            csv_writer.write_record(&string_row)?;
        }
        
        csv_writer.flush()?;
        Ok(())
    }
}

impl Default for CsvOutput {
    fn default() -> Self {
        Self::new(b',', b'"', true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_output_creation() {
        let _csv = CsvOutput::new(b',', b'"', true);
        let _csv_tab = CsvOutput::new(b'\t', b'"', false);
    }

    #[test]
    fn test_empty_sheet_write() {
        let csv_output = CsvOutput::default();
        let empty_sheet = Sheet {
            name: "Empty".to_string(),
            data: vec![],
        };
        
        let mut output = Vec::new();
        csv_output.write_sheet(&empty_sheet, &mut output).unwrap();
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn test_single_row_write() {
        let csv_output = CsvOutput::default();
        let sheet = Sheet {
            name: "Test".to_string(),
            data: vec![vec!["Name".to_string(), "Age".to_string()]],
        };
        
        let mut output = Vec::new();
        csv_output.write_sheet(&sheet, &mut output).unwrap();
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("Name,Age"));
    }

    #[test]
    fn test_multiple_rows_write() {
        let csv_output = CsvOutput::default();
        let sheet = Sheet {
            name: "Test".to_string(),
            data: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "25".to_string()],
                vec!["Jane".to_string(), "30".to_string()],
            ],
        };
        
        let mut output = Vec::new();
        csv_output.write_sheet(&sheet, &mut output).unwrap();
        
        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("Name,Age"));
        assert!(result.contains("John,25"));
        assert!(result.contains("Jane,30"));
    }
}