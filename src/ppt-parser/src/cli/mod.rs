use crate::error::{PptParserError, Result};
use crate::output::OutputFormat;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "ppt-parser",
    about = "A cross-platform PowerPoint parser that converts PPT/PPTX files to multiple text formats",
    long_about = "A high-performance PowerPoint parser written in Rust that can convert PPT and PPTX files to various text formats including plain text, JSON, Markdown, and HTML. Supports metadata extraction, table parsing, and slide-specific processing.",
    version
)]
pub struct Args {
    /// Input PPT/PPTX file path
    #[arg(value_name = "INPUT_FILE")]
    pub input_file: PathBuf,

    /// Output file path (stdout if not specified)
    #[arg(short, long, value_name = "OUTPUT")]
    pub output: Option<PathBuf>,

    /// Specific slide number to process (1-based)
    #[arg(short = 's', long, value_name = "SLIDE")]
    pub slide: Option<usize>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    pub format: Format,

    /// Include presentation metadata in output
    #[arg(long)]
    pub metadata: bool,

    /// Pretty print JSON output
    #[arg(long)]
    pub pretty: bool,

    /// Include slide numbers in markdown output
    #[arg(long, default_value_t = true)]
    pub slide_numbers: bool,

    /// Include CSS styles in HTML output
    #[arg(long, default_value_t = true)]
    pub css: bool,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Format {
    /// Plain text output
    Text,
    /// JSON structured output
    Json,
    /// Markdown formatted output
    Markdown,
    /// HTML formatted output
    Html,
}

impl Args {
    pub fn validate(&self) -> Result<()> {
        // Check if input file exists
        if !self.input_file.exists() {
            return Err(PptParserError::FileNotFound(
                self.input_file.display().to_string(),
            ));
        }

        // Check file extension
        let extension = self
            .input_file
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "ppt" | "pptx" => {}
            _ => {
                return Err(PptParserError::UnsupportedFormat(format!(
                    "Unsupported file extension: '{}'. Supported formats: .ppt, .pptx",
                    extension
                )));
            }
        }

        // Validate slide number if specified
        if let Some(slide_num) = self.slide {
            if slide_num == 0 {
                return Err(PptParserError::InvalidSlideRange(
                    "Slide numbers start from 1".to_string(),
                ));
            }
        }

        // Check conflicting options
        if self.format == Format::Text && self.pretty {
            return Err(PptParserError::ParsingError(
                "Pretty printing is not applicable to text format".to_string(),
            ));
        }

        if self.format != Format::Json && self.pretty {
            return Err(PptParserError::ParsingError(
                "Pretty printing is only available for JSON format".to_string(),
            ));
        }

        if self.format != Format::Markdown && !self.slide_numbers {
            return Err(PptParserError::ParsingError(
                "Slide numbers option is only available for Markdown format".to_string(),
            ));
        }

        if self.format != Format::Html && !self.css {
            return Err(PptParserError::ParsingError(
                "CSS option is only available for HTML format".to_string(),
            ));
        }

        Ok(())
    }

    pub fn get_output_format(&self) -> OutputFormat {
        match self.format {
            Format::Text => OutputFormat::Text,
            Format::Json => {
                if self.metadata && self.pretty {
                    OutputFormat::json_pretty_with_metadata()
                } else if self.metadata {
                    OutputFormat::json_with_metadata()
                } else if self.pretty {
                    OutputFormat::json_pretty()
                } else {
                    OutputFormat::json()
                }
            }
            Format::Markdown => OutputFormat::Markdown {
                include_metadata: self.metadata,
                include_slide_numbers: self.slide_numbers,
            },
            Format::Html => OutputFormat::Html {
                include_metadata: self.metadata,
                include_css: self.css,
            },
        }
    }
}

impl PartialEq for Format {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Format::Text, Format::Text)
                | (Format::Json, Format::Json)
                | (Format::Markdown, Format::Markdown)
                | (Format::Html, Format::Html)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_args_validation_missing_file() {
        let args = Args {
            input_file: PathBuf::from("nonexistent.pptx"),
            output: None,
            slide: None,
            format: Format::Text,
            metadata: false,
            pretty: false,
            slide_numbers: true,
            css: true,
            verbose: false,
        };

        let result = args.validate();
        assert!(matches!(result, Err(PptParserError::FileNotFound(_))));
    }

    #[test]
    fn test_args_validation_unsupported_format() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().with_extension("txt");
        std::fs::write(&file_path, b"dummy content").unwrap();

        let args = Args {
            input_file: file_path.clone(),
            output: None,
            slide: None,
            format: Format::Text,
            metadata: false,
            pretty: false,
            slide_numbers: true,
            css: true,
            verbose: false,
        };

        let result = args.validate();
        
        // Clean up
        let _ = std::fs::remove_file(&file_path);
        
        assert!(matches!(result, Err(PptParserError::UnsupportedFormat(_))));
    }

    #[test]
    fn test_args_validation_invalid_slide_number() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut file_path = temp_file.path().to_path_buf();
        file_path.set_extension("pptx");
        std::fs::write(&file_path, b"dummy content").unwrap();

        let args = Args {
            input_file: file_path,
            output: None,
            slide: Some(0),
            format: Format::Text,
            metadata: false,
            pretty: false,
            slide_numbers: true,
            css: true,
            verbose: false,
        };

        let result = args.validate();
        assert!(matches!(result, Err(PptParserError::InvalidSlideRange(_))));
    }

    #[test]
    fn test_args_validation_conflicting_options() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut file_path = temp_file.path().to_path_buf();
        file_path.set_extension("pptx");
        std::fs::write(&file_path, b"dummy content").unwrap();

        // Test pretty with text format
        let args = Args {
            input_file: file_path.clone(),
            output: None,
            slide: None,
            format: Format::Text,
            metadata: false,
            pretty: true,
            slide_numbers: true,
            css: true,
            verbose: false,
        };

        let result = args.validate();
        assert!(matches!(result, Err(PptParserError::ParsingError(_))));

        // Test pretty with markdown format
        let args = Args {
            input_file: file_path,
            output: None,
            slide: None,
            format: Format::Markdown,
            metadata: false,
            pretty: true,
            slide_numbers: true,
            css: true,
            verbose: false,
        };

        let result = args.validate();
        assert!(matches!(result, Err(PptParserError::ParsingError(_))));
    }

    #[test]
    fn test_get_output_format() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut file_path = temp_file.path().to_path_buf();
        file_path.set_extension("pptx");
        std::fs::write(&file_path, b"dummy content").unwrap();

        // Test JSON with metadata and pretty
        let args = Args {
            input_file: file_path.clone(),
            output: None,
            slide: None,
            format: Format::Json,
            metadata: true,
            pretty: true,
            slide_numbers: true,
            css: true,
            verbose: false,
        };

        let format = args.get_output_format();
        assert!(matches!(
            format,
            OutputFormat::Json {
                pretty: true,
                include_metadata: true
            }
        ));

        // Test Markdown with custom options
        let args = Args {
            input_file: file_path.clone(),
            output: None,
            slide: None,
            format: Format::Markdown,
            metadata: true,
            pretty: false,
            slide_numbers: false,
            css: true,
            verbose: false,
        };

        let format = args.get_output_format();
        assert!(matches!(
            format,
            OutputFormat::Markdown {
                include_metadata: true,
                include_slide_numbers: false
            }
        ));

        // Test HTML with CSS
        let args = Args {
            input_file: file_path,
            output: None,
            slide: None,
            format: Format::Html,
            metadata: false,
            pretty: false,
            slide_numbers: true,
            css: false,
            verbose: false,
        };

        let format = args.get_output_format();
        assert!(matches!(
            format,
            OutputFormat::Html {
                include_metadata: false,
                include_css: false
            }
        ));
    }

    #[test]
    fn test_format_equality() {
        assert_eq!(Format::Text, Format::Text);
        assert_eq!(Format::Json, Format::Json);
        assert_eq!(Format::Markdown, Format::Markdown);
        assert_eq!(Format::Html, Format::Html);
        assert_ne!(Format::Text, Format::Json);
    }
}