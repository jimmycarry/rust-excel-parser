use crate::error::Result;
use crate::parser::{ExcelData, Sheet};
use std::io::Write;

pub mod csv;
pub mod json;
pub mod table;

pub use csv::CsvOutput;
pub use json::JsonOutput;
pub use table::TableOutput;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Csv {
        delimiter: u8,
        quote_char: u8,
        has_headers: bool,
    },
    Json {
        pretty: bool,
    },
    Table {
        max_width: Option<usize>,
        borders: bool,
    },
}

impl OutputFormat {
    pub fn csv() -> Self {
        Self::Csv {
            delimiter: b',',
            quote_char: b'"',
            has_headers: true,
        }
    }

    pub fn csv_with_delimiter(delimiter: u8) -> Self {
        Self::Csv {
            delimiter,
            quote_char: b'"',
            has_headers: true,
        }
    }

    pub fn csv_no_headers() -> Self {
        Self::Csv {
            delimiter: b',',
            quote_char: b'"',
            has_headers: false,
        }
    }

    pub fn json() -> Self {
        Self::Json { pretty: false }
    }

    pub fn json_pretty() -> Self {
        Self::Json { pretty: true }
    }

    pub fn table() -> Self {
        Self::Table {
            max_width: None,
            borders: true,
        }
    }

    pub fn table_no_borders() -> Self {
        Self::Table {
            max_width: None,
            borders: false,
        }
    }

    pub fn table_with_width(max_width: usize) -> Self {
        Self::Table {
            max_width: Some(max_width),
            borders: true,
        }
    }
}

pub trait OutputWriter {
    fn write_excel_data<W: Write>(&self, data: &ExcelData, writer: &mut W) -> Result<()>;
    fn write_sheet<W: Write>(&self, sheet: &Sheet, writer: &mut W) -> Result<()>;
}

pub struct OutputProcessor;

impl OutputProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process<W: Write>(
        &self,
        data: &ExcelData,
        format: &OutputFormat,
        writer: &mut W,
    ) -> Result<()> {
        match format {
            OutputFormat::Csv {
                delimiter,
                quote_char,
                has_headers,
            } => {
                let csv_output = CsvOutput::new(*delimiter, *quote_char, *has_headers);
                csv_output.write_excel_data(data, writer)
            }
            OutputFormat::Json { pretty } => {
                let json_output = JsonOutput::new(*pretty);
                json_output.write_excel_data(data, writer)
            }
            OutputFormat::Table { max_width, borders } => {
                let table_output = TableOutput::new(*max_width, *borders);
                table_output.write_excel_data(data, writer)
            }
        }
    }

    pub fn process_sheet<W: Write>(
        &self,
        sheet: &Sheet,
        format: &OutputFormat,
        writer: &mut W,
    ) -> Result<()> {
        match format {
            OutputFormat::Csv {
                delimiter,
                quote_char,
                has_headers,
            } => {
                let csv_output = CsvOutput::new(*delimiter, *quote_char, *has_headers);
                csv_output.write_sheet(sheet, writer)
            }
            OutputFormat::Json { pretty } => {
                let json_output = JsonOutput::new(*pretty);
                json_output.write_sheet(sheet, writer)
            }
            OutputFormat::Table { max_width, borders } => {
                let table_output = TableOutput::new(*max_width, *borders);
                table_output.write_sheet(sheet, writer)
            }
        }
    }
}

impl Default for OutputProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_creation() {
        let _csv = OutputFormat::csv();
        let _csv_tab = OutputFormat::csv_with_delimiter(b'\t');
        let _csv_no_headers = OutputFormat::csv_no_headers();
    }
}
