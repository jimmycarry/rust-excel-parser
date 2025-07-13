use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input PDF file
    #[arg(help = "Input PDF file path")]
    pub input: PathBuf,

    /// Output file (default: stdout)
    #[arg(short, long, help = "Output file path (default: stdout)")]
    pub output: Option<PathBuf>,

    /// Specific page to process
    #[arg(short, long, help = "Specific page number to process (1-based)")]
    pub page: Option<usize>,

    /// Output format
    #[arg(
        short = 'f',
        long,
        default_value = "text",
        help = "Output format: text, json, markdown, csv"
    )]
    pub format: String,

    /// Extract tables only
    #[arg(long, help = "Extract tables only")]
    pub tables_only: bool,

    /// Include metadata in output
    #[arg(long, help = "Include PDF metadata in output")]
    pub metadata: bool,

    /// Pretty print JSON output
    #[arg(long, help = "Pretty print JSON output")]
    pub pretty: bool,

    /// Custom delimiter for CSV output
    #[arg(
        short,
        long,
        default_value = ",",
        help = "Custom delimiter for CSV output"
    )]
    pub delimiter: String,

    /// Don't treat first row as header in CSV
    #[arg(short = 'n', long, help = "Don't treat first row as header in CSV")]
    pub no_header: bool,

    /// Enable verbose output
    #[arg(short, long, help = "Enable verbose output")]
    pub verbose: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn get_delimiter(&self) -> u8 {
        self.delimiter.chars().next().unwrap_or(',') as u8
    }

    pub fn has_headers(&self) -> bool {
        !self.no_header
    }

    pub fn get_output_format(&self) -> Result<crate::output::OutputFormat, String> {
        match self.format.to_lowercase().as_str() {
            "text" => Ok(crate::output::OutputFormat::text()),
            "json" => {
                if self.metadata && self.pretty {
                    Ok(crate::output::OutputFormat::json_pretty_with_metadata())
                } else if self.metadata {
                    Ok(crate::output::OutputFormat::json_with_metadata())
                } else if self.pretty {
                    Ok(crate::output::OutputFormat::json_pretty())
                } else {
                    Ok(crate::output::OutputFormat::json())
                }
            }
            "markdown" => {
                if self.metadata {
                    Ok(crate::output::OutputFormat::markdown_with_metadata())
                } else {
                    Ok(crate::output::OutputFormat::markdown())
                }
            }
            "csv" => {
                if self.no_header {
                    Ok(crate::output::OutputFormat::csv_no_headers())
                } else {
                    Ok(crate::output::OutputFormat::csv_with_delimiter(
                        self.get_delimiter(),
                    ))
                }
            }
            _ => Err(format!("Unsupported output format: {}", self.format)),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // Check if input file exists
        if !self.input.exists() {
            return Err(format!(
                "Input file does not exist: {}",
                self.input.display()
            ));
        }

        // Check if input file has valid extension
        let extension = self
            .input
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        if extension != "pdf" {
            return Err(format!("Unsupported file format: {}", extension));
        }

        // Validate format
        match self.format.to_lowercase().as_str() {
            "text" | "json" | "markdown" | "csv" => {}
            _ => return Err(format!("Unsupported output format: {}", self.format)),
        }

        // Validate page number
        if let Some(page) = self.page {
            if page == 0 {
                return Err("Page numbers start from 1".to_string());
            }
        }

        // Validate delimiter (only for CSV)
        if self.format.to_lowercase() == "csv" {
            if self.delimiter.is_empty() {
                return Err("Delimiter cannot be empty".to_string());
            }

            if self.delimiter.len() > 1 {
                return Err("Delimiter must be a single character".to_string());
            }
        }

        // Validate conflicting options
        if self.tables_only && self.page.is_some() {
            return Err("Cannot use --tables-only with --page option".to_string());
        }

        if self.metadata && self.format.to_lowercase() == "text" {
            return Err("Metadata option is not supported with text format".to_string());
        }

        if self.pretty && self.format.to_lowercase() != "json" {
            return Err("Pretty print option is only available for JSON format".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delimiter_parsing() {
        let args = Args {
            input: PathBuf::from("test.pdf"),
            output: None,
            page: None,
            format: "csv".to_string(),
            tables_only: false,
            metadata: false,
            pretty: false,
            delimiter: ",".to_string(),
            no_header: false,
            verbose: false,
        };

        assert_eq!(args.get_delimiter(), b',');
        assert!(args.has_headers());
    }

    #[test]
    fn test_tab_delimiter() {
        let args = Args {
            input: PathBuf::from("test.pdf"),
            output: None,
            page: None,
            format: "csv".to_string(),
            tables_only: false,
            metadata: false,
            pretty: false,
            delimiter: "\t".to_string(),
            no_header: false,
            verbose: false,
        };

        assert_eq!(args.get_delimiter(), b'\t');
    }

    #[test]
    fn test_no_header_flag() {
        let args = Args {
            input: PathBuf::from("test.pdf"),
            output: None,
            page: None,
            format: "csv".to_string(),
            tables_only: false,
            metadata: false,
            pretty: false,
            delimiter: ",".to_string(),
            no_header: true,
            verbose: false,
        };

        assert!(!args.has_headers());
    }

    #[test]
    fn test_json_format_variants() {
        let args = Args {
            input: PathBuf::from("test.pdf"),
            output: None,
            page: None,
            format: "json".to_string(),
            tables_only: false,
            metadata: true,
            pretty: true,
            delimiter: ",".to_string(),
            no_header: false,
            verbose: false,
        };

        let format = args.get_output_format().unwrap();
        match format {
            crate::output::OutputFormat::Json {
                pretty,
                include_metadata,
            } => {
                assert!(pretty);
                assert!(include_metadata);
            }
            _ => panic!("Expected JSON format"),
        }
    }

    #[test]
    fn test_validation_invalid_page() {
        let args = Args {
            input: PathBuf::from("test.pdf"),
            output: None,
            page: Some(0),
            format: "text".to_string(),
            tables_only: false,
            metadata: false,
            pretty: false,
            delimiter: ",".to_string(),
            no_header: false,
            verbose: false,
        };

        assert!(args.validate().is_err());
    }

    #[test]
    fn test_validation_conflicting_options() {
        let args = Args {
            input: PathBuf::from("test.pdf"),
            output: None,
            page: Some(1),
            format: "text".to_string(),
            tables_only: true,
            metadata: false,
            pretty: false,
            delimiter: ",".to_string(),
            no_header: false,
            verbose: false,
        };

        assert!(args.validate().is_err());
    }
}
