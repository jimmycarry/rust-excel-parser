use crate::error::Result;
use crate::parser::{DocData, DocSection};
use std::io::Write;

pub mod text;
pub mod markdown;
pub mod json;

pub use text::TextOutput;
pub use markdown::MarkdownOutput;
pub use json::JsonOutput;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Text {
        preserve_formatting: bool,
        include_metadata: bool,
        line_numbers: bool,
    },
    Markdown {
        preserve_structure: bool,
        include_metadata: bool,
    },
    Json {
        pretty: bool,
        include_formatting: bool,
    },
}

impl OutputFormat {
    pub fn text() -> Self {
        Self::Text {
            preserve_formatting: false,
            include_metadata: false,
            line_numbers: false,
        }
    }

    pub fn text_with_metadata() -> Self {
        Self::Text {
            preserve_formatting: false,
            include_metadata: true,
            line_numbers: false,
        }
    }

    pub fn text_with_line_numbers() -> Self {
        Self::Text {
            preserve_formatting: false,
            include_metadata: false,
            line_numbers: true,
        }
    }

    pub fn markdown() -> Self {
        Self::Markdown {
            preserve_structure: true,
            include_metadata: false,
        }
    }

    pub fn markdown_with_metadata() -> Self {
        Self::Markdown {
            preserve_structure: true,
            include_metadata: true,
        }
    }

    pub fn json() -> Self {
        Self::Json {
            pretty: false,
            include_formatting: false,
        }
    }

    pub fn json_pretty() -> Self {
        Self::Json {
            pretty: true,
            include_formatting: true,
        }
    }
}

pub trait OutputWriter {
    fn write_doc_data<W: Write>(&self, data: &DocData, writer: &mut W) -> Result<()>;
    fn write_sections<W: Write>(&self, sections: &[DocSection], writer: &mut W) -> Result<()>;
}

pub struct OutputProcessor;

impl OutputProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process<W: Write>(
        &self,
        data: &DocData,
        format: &OutputFormat,
        writer: &mut W,
    ) -> Result<()> {
        match format {
            OutputFormat::Text { preserve_formatting, include_metadata, line_numbers } => {
                let text_output = TextOutput::new(*preserve_formatting, *include_metadata, *line_numbers);
                text_output.write_doc_data(data, writer)
            }
            OutputFormat::Markdown { preserve_structure, include_metadata } => {
                let markdown_output = MarkdownOutput::new(*preserve_structure, *include_metadata);
                markdown_output.write_doc_data(data, writer)
            }
            OutputFormat::Json { pretty, include_formatting } => {
                let json_output = JsonOutput::new(*pretty, *include_formatting);
                json_output.write_doc_data(data, writer)
            }
        }
    }

    pub fn process_sections<W: Write>(
        &self,
        sections: &[DocSection],
        format: &OutputFormat,
        writer: &mut W,
    ) -> Result<()> {
        match format {
            OutputFormat::Text { preserve_formatting, include_metadata: _, line_numbers } => {
                let text_output = TextOutput::new(*preserve_formatting, false, *line_numbers);
                text_output.write_sections(sections, writer)
            }
            OutputFormat::Markdown { preserve_structure, include_metadata: _ } => {
                let markdown_output = MarkdownOutput::new(*preserve_structure, false);
                markdown_output.write_sections(sections, writer)
            }
            OutputFormat::Json { pretty, include_formatting } => {
                let json_output = JsonOutput::new(*pretty, *include_formatting);
                json_output.write_sections(sections, writer)
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
        let _text_meta = OutputFormat::text_with_metadata();
        let _text_lines = OutputFormat::text_with_line_numbers();
        let _markdown = OutputFormat::markdown();
        let _json = OutputFormat::json();
        let _json_pretty = OutputFormat::json_pretty();
    }

    #[test]
    fn test_output_processor_creation() {
        let _processor = OutputProcessor::new();
        let _processor_default = OutputProcessor::default();
    }
}