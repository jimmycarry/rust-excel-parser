use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input Excel file (.xlsx, .xlsm, .xlsb, .xls)
    #[arg(help = "Input Excel file path")]
    pub input: PathBuf,

    /// Output file (default: stdout)
    #[arg(short, long, help = "Output file path (default: stdout)")]
    pub output: Option<PathBuf>,

    /// Specific sheet name to process
    #[arg(short, long, help = "Specific sheet name to process")]
    pub sheet: Option<String>,

    /// Output format
    #[arg(short = 'f', long, default_value = "csv", help = "Output format: csv, json, table")]
    pub format: String,

    /// Custom delimiter for CSV output
    #[arg(short, long, default_value = ",", help = "Custom delimiter for CSV output")]
    pub delimiter: String,

    /// Don't treat first row as header
    #[arg(short = 'n', long, help = "Don't treat first row as header")]
    pub no_header: bool,

    /// Pretty print JSON output
    #[arg(long, help = "Pretty print JSON output")]
    pub pretty: bool,

    /// Maximum width for table output
    #[arg(long, help = "Maximum width for table output")]
    pub max_width: Option<usize>,

    /// Hide borders in table output
    #[arg(long, help = "Hide borders in table output")]
    pub no_borders: bool,

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
            "csv" => {
                if self.no_header {
                    Ok(crate::output::OutputFormat::csv_no_headers())
                } else {
                    Ok(crate::output::OutputFormat::csv_with_delimiter(self.get_delimiter()))
                }
            }
            "json" => {
                if self.pretty {
                    Ok(crate::output::OutputFormat::json_pretty())
                } else {
                    Ok(crate::output::OutputFormat::json())
                }
            }
            "table" => {
                if let Some(max_width) = self.max_width {
                    Ok(crate::output::OutputFormat::table_with_width(max_width))
                } else if self.no_borders {
                    Ok(crate::output::OutputFormat::table_no_borders())
                } else {
                    Ok(crate::output::OutputFormat::table())
                }
            }
            _ => Err(format!("Unsupported output format: {}", self.format)),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // Check if input file exists
        if !self.input.exists() {
            return Err(format!("Input file does not exist: {}", self.input.display()));
        }

        // Check if input file has valid extension
        let extension = self.input
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "xlsx" | "xlsm" | "xlsb" | "xls" => {}
            _ => return Err(format!("Unsupported file format: {}", extension)),
        }

        // Validate format
        match self.format.to_lowercase().as_str() {
            "csv" | "json" | "table" => {}
            _ => return Err(format!("Unsupported output format: {}", self.format)),
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delimiter_parsing() {
        let args = Args {
            input: PathBuf::from("test.xlsx"),
            output: None,
            sheet: None,
            format: "csv".to_string(),
            delimiter: ",".to_string(),
            no_header: false,
            pretty: false,
            max_width: None,
            no_borders: false,
            verbose: false,
        };
        
        assert_eq!(args.get_delimiter(), b',');
        assert!(args.has_headers());
    }

    #[test]
    fn test_tab_delimiter() {
        let args = Args {
            input: PathBuf::from("test.xlsx"),
            output: None,
            sheet: None,
            format: "csv".to_string(),
            delimiter: "\t".to_string(),
            no_header: false,
            pretty: false,
            max_width: None,
            no_borders: false,
            verbose: false,
        };
        
        assert_eq!(args.get_delimiter(), b'\t');
    }

    #[test]
    fn test_no_header_flag() {
        let args = Args {
            input: PathBuf::from("test.xlsx"),
            output: None,
            sheet: None,
            format: "csv".to_string(),
            delimiter: ",".to_string(),
            no_header: true,
            pretty: false,
            max_width: None,
            no_borders: false,
            verbose: false,
        };
        
        assert!(!args.has_headers());
    }
}