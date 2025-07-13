use crate::error::Result;
use crate::parser::{PptData, Slide};
use serde::{Deserialize, Serialize};
use std::io::Write;

pub mod text;
pub mod json;
pub mod markdown;
pub mod html;

pub use text::TextOutput;
pub use json::JsonOutput;
pub use markdown::MarkdownOutput;
pub use html::HtmlOutput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Text,
    Json {
        pretty: bool,
        include_metadata: bool,
    },
    Markdown {
        include_metadata: bool,
        include_slide_numbers: bool,
    },
    Html {
        include_metadata: bool,
        include_css: bool,
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
            include_slide_numbers: true,
        }
    }

    pub fn markdown_with_metadata() -> Self {
        Self::Markdown {
            include_metadata: true,
            include_slide_numbers: true,
        }
    }

    pub fn html() -> Self {
        Self::Html {
            include_metadata: false,
            include_css: true,
        }
    }

    pub fn html_with_metadata() -> Self {
        Self::Html {
            include_metadata: true,
            include_css: true,
        }
    }
}

pub trait OutputWriter {
    fn write_ppt_data<W: Write>(&self, data: &PptData, writer: &mut W) -> Result<()>;
    fn write_slide<W: Write>(&self, slide: &Slide, writer: &mut W) -> Result<()>;
}

pub struct OutputProcessor;

impl OutputProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process<W: Write>(
        &self,
        data: &PptData,
        format: &OutputFormat,
        writer: &mut W,
    ) -> Result<()> {
        match format {
            OutputFormat::Text => {
                let output = TextOutput::new();
                output.write_ppt_data(data, writer)
            }
            OutputFormat::Json {
                pretty,
                include_metadata,
            } => {
                let output = JsonOutput::new(*pretty, *include_metadata);
                output.write_ppt_data(data, writer)
            }
            OutputFormat::Markdown {
                include_metadata,
                include_slide_numbers,
            } => {
                let output = MarkdownOutput::new(*include_metadata, *include_slide_numbers);
                output.write_ppt_data(data, writer)
            }
            OutputFormat::Html {
                include_metadata,
                include_css,
            } => {
                let output = HtmlOutput::new(*include_metadata, *include_css);
                output.write_ppt_data(data, writer)
            }
        }
    }

    pub fn process_slide<W: Write>(
        &self,
        slide: &Slide,
        format: &OutputFormat,
        writer: &mut W,
    ) -> Result<()> {
        match format {
            OutputFormat::Text => {
                let output = TextOutput::new();
                output.write_slide(slide, writer)
            }
            OutputFormat::Json {
                pretty,
                include_metadata: _,
            } => {
                let output = JsonOutput::new(*pretty, false); // No metadata for single slide
                output.write_slide(slide, writer)
            }
            OutputFormat::Markdown {
                include_metadata: _,
                include_slide_numbers,
            } => {
                let output = MarkdownOutput::new(false, *include_slide_numbers); // No metadata for single slide
                output.write_slide(slide, writer)
            }
            OutputFormat::Html {
                include_metadata: _,
                include_css,
            } => {
                let output = HtmlOutput::new(false, *include_css); // No metadata for single slide
                output.write_slide(slide, writer)
            }
        }
    }
}

impl Default for OutputProcessor {
    fn default() -> Self {
        Self::new()
    }
}