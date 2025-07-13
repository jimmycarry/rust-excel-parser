use crate::error::Result;
use crate::parser::{PdfData, Page, Table};
use std::io::Write;

pub mod text;
pub mod json;
pub mod markdown;
pub mod csv;

pub use text::TextOutput;
pub use json::JsonOutput;
pub use markdown::MarkdownOutput;
pub use csv::CsvOutput;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Text,
    Json {
        pretty: bool,
        include_metadata: bool,
    },
    Markdown {
        include_metadata: bool,
    },
    Csv {
        delimiter: u8,
        quote_char: u8,
        has_headers: bool,
    },
}

impl OutputFormat {
    pub fn text() -> Self {
        Self::Text
    }

    pub fn json() -> Self {
        Self::Json {
            pretty: false,
            include_metadata: false,
        }
    }

    pub fn json_pretty() -> Self {
        Self::Json {
            pretty: true,
            include_metadata: false,
        }
    }

    pub fn json_with_metadata() -> Self {
        Self::Json {
            pretty: false,
            include_metadata: true,
        }
    }

    pub fn json_pretty_with_metadata() -> Self {
        Self::Json {
            pretty: true,
            include_metadata: true,
        }
    }

    pub fn markdown() -> Self {
        Self::Markdown {
            include_metadata: false,
        }
    }

    pub fn markdown_with_metadata() -> Self {
        Self::Markdown {
            include_metadata: true,
        }
    }

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
}

pub trait OutputWriter {
    fn write_pdf_data<W: Write>(&self, data: &PdfData, writer: &mut W) -> Result<()>;
    fn write_page<W: Write>(&self, page: &Page, writer: &mut W) -> Result<()>;
    fn write_tables<W: Write>(&self, tables: &[Table], writer: &mut W) -> Result<()>;
}

pub struct OutputProcessor;

impl OutputProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process<W: Write>(
        &self,
        data: &PdfData,
        format: &OutputFormat,
        writer: &mut W,
    ) -> Result<()> {
        match format {
            OutputFormat::Text => {
                let text_output = TextOutput::new();
                text_output.write_pdf_data(data, writer)
            }
            OutputFormat::Json { pretty, include_metadata } => {
                let json_output = JsonOutput::new(*pretty, *include_metadata);
                json_output.write_pdf_data(data, writer)
            }
            OutputFormat::Markdown { include_metadata } => {
                let markdown_output = MarkdownOutput::new(*include_metadata);
                markdown_output.write_pdf_data(data, writer)
            }
            OutputFormat::Csv {
                delimiter,
                quote_char,
                has_headers,
            } => {
                let csv_output = CsvOutput::new(*delimiter, *quote_char, *has_headers);
                csv_output.write_pdf_data(data, writer)
            }
        }
    }

    pub fn process_page<W: Write>(
        &self,
        page: &Page,
        format: &OutputFormat,
        writer: &mut W,
    ) -> Result<()> {
        match format {
            OutputFormat::Text => {
                let text_output = TextOutput::new();
                text_output.write_page(page, writer)
            }
            OutputFormat::Json { pretty, include_metadata: _ } => {
                let json_output = JsonOutput::new(*pretty, false);
                json_output.write_page(page, writer)
            }
            OutputFormat::Markdown { include_metadata: _ } => {
                let markdown_output = MarkdownOutput::new(false);
                markdown_output.write_page(page, writer)
            }
            OutputFormat::Csv {
                delimiter,
                quote_char,
                has_headers,
            } => {
                let csv_output = CsvOutput::new(*delimiter, *quote_char, *has_headers);
                csv_output.write_page(page, writer)
            }
        }
    }

    pub fn process_tables<W: Write>(
        &self,
        tables: &[Table],
        format: &OutputFormat,
        writer: &mut W,
    ) -> Result<()> {
        match format {
            OutputFormat::Text => {
                let text_output = TextOutput::new();
                text_output.write_tables(tables, writer)
            }
            OutputFormat::Json { pretty, include_metadata: _ } => {
                let json_output = JsonOutput::new(*pretty, false);
                json_output.write_tables(tables, writer)
            }
            OutputFormat::Markdown { include_metadata: _ } => {
                let markdown_output = MarkdownOutput::new(false);
                markdown_output.write_tables(tables, writer)
            }
            OutputFormat::Csv {
                delimiter,
                quote_char,
                has_headers,
            } => {
                let csv_output = CsvOutput::new(*delimiter, *quote_char, *has_headers);
                csv_output.write_tables(tables, writer)
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
        let _text = OutputFormat::text();
        let _json = OutputFormat::json();
        let _json_pretty = OutputFormat::json_pretty();
        let _markdown = OutputFormat::markdown();
        let _csv = OutputFormat::csv();
        let _csv_tab = OutputFormat::csv_with_delimiter(b'\t');
        let _csv_no_headers = OutputFormat::csv_no_headers();
    }
}