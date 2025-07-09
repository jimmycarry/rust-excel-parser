use clap::Parser;
use std::path::PathBuf;
use crate::error::{DocParserError, Result};
use glob;

#[derive(Parser, Debug)]
#[command(
    name = "doc-parser",
    author = "Claude Code", 
    version = "0.1.0",
    about = "A cross-platform DOC/DOCX parser that extracts text content"
)]
pub struct Args {
    /// Input DOC/DOCX file
    #[arg(help = "Input DOC/DOCX file path")]
    pub input: PathBuf,

    /// Output file (default: stdout)
    #[arg(short, long, help = "Output file path (default: stdout)")]
    pub output: Option<PathBuf>,

    /// Output format
    #[arg(short = 'f', long, default_value = "text", 
          help = "Output format: text, markdown, json")]
    pub format: String,

    /// Include metadata in output
    #[arg(long, help = "Include document metadata in output")]
    pub metadata: bool,

    /// Preserve text formatting (for text output)
    #[arg(long, help = "Preserve text formatting")]
    pub preserve_formatting: bool,

    /// Pretty print JSON output
    #[arg(long, help = "Pretty print JSON output")]
    pub pretty: bool,

    /// Add line numbers (for text output)
    #[arg(long, help = "Add line numbers to text output")]
    pub line_numbers: bool,

    /// Extract only plain text (fastest)
    #[arg(long, help = "Extract only plain text without structure")]
    pub text_only: bool,

    /// Enable verbose output
    #[arg(short, long, help = "Enable verbose output")]
    pub verbose: bool,

    /// Process multiple files (directory or glob pattern)
    #[arg(short, long, help = "Process multiple files from directory or glob pattern")]
    pub batch: Option<String>,

    /// Output directory for batch processing
    #[arg(long, help = "Output directory for batch processing (default: current directory)")]
    pub output_dir: Option<PathBuf>,

    /// Overwrite existing output files
    #[arg(long, help = "Overwrite existing output files")]
    pub overwrite: bool,

    /// Maximum number of files to process
    #[arg(long, help = "Maximum number of files to process (default: unlimited)")]
    pub max_files: Option<usize>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn get_output_format(&self) -> Result<crate::output::OutputFormat> {
        match self.format.to_lowercase().as_str() {
            "text" => {
                Ok(crate::output::OutputFormat::Text {
                    preserve_formatting: self.preserve_formatting,
                    include_metadata: self.metadata,
                    line_numbers: self.line_numbers,
                })
            }
            "markdown" | "md" => {
                Ok(crate::output::OutputFormat::Markdown {
                    preserve_structure: !self.text_only,
                    include_metadata: self.metadata,
                })
            }
            "json" => {
                Ok(crate::output::OutputFormat::Json {
                    pretty: self.pretty,
                    include_formatting: self.preserve_formatting,
                })
            }
            _ => Err(DocParserError::InvalidConfiguration {
                details: format!("Unsupported output format: '{}'. Supported formats: text, markdown, json", self.format)
            }),
        }
    }

    pub fn validate(&self) -> Result<()> {
        // 如果是批处理模式，验证批处理参数
        if let Some(batch_pattern) = &self.batch {
            return self.validate_batch_mode(batch_pattern);
        }

        // 单文件模式验证
        self.validate_single_file_mode()
    }

    fn validate_single_file_mode(&self) -> Result<()> {
        // 检查输入文件是否存在
        if !self.input.exists() {
            return Err(DocParserError::FileNotFound {
                file: self.input.display().to_string(),
            });
        }

        // 检查文件扩展名
        let extension = self.input
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "docx" | "doc" => {}
            _ => return Err(DocParserError::UnsupportedFormat {
                format: extension,
                file: self.input.display().to_string(),
            }),
        }

        self.validate_common_options()
    }

    fn validate_batch_mode(&self, batch_pattern: &str) -> Result<()> {
        // 批处理模式不能指定单个输出文件
        if self.output.is_some() {
            return Err(DocParserError::InvalidConfiguration {
                details: "Cannot specify --output with --batch. Use --output-dir instead.".to_string(),
            });
        }

        // 验证输出目录
        if let Some(output_dir) = &self.output_dir {
            if !output_dir.exists() {
                return Err(DocParserError::OutputDirectoryNotFound {
                    directory: output_dir.display().to_string(),
                });
            }
        }

        // 验证最大文件数
        if let Some(max_files) = self.max_files {
            if max_files == 0 {
                return Err(DocParserError::InvalidConfiguration {
                    details: "max-files must be greater than 0".to_string(),
                });
            }
        }

        self.validate_common_options()
    }

    fn validate_common_options(&self) -> Result<()> {
        // 验证输出格式
        match self.format.to_lowercase().as_str() {
            "text" | "markdown" | "md" | "json" => {}
            _ => return Err(DocParserError::InvalidConfiguration {
                details: format!("Unsupported output format: '{}'. Supported formats: text, markdown, json", self.format)
            }),
        }

        // 检查输出文件路径是否有效（单文件模式）
        if let Some(output_path) = &self.output {
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    return Err(DocParserError::OutputDirectoryNotFound {
                        directory: parent.display().to_string(),
                    });
                }
            }
        }

        // 验证格式特定的选项组合
        if self.line_numbers && self.format.to_lowercase() != "text" {
            return Err(DocParserError::InvalidConfiguration {
                details: "Line numbers are only supported with text output format".to_string(),
            });
        }

        if self.pretty && self.format.to_lowercase() != "json" {
            return Err(DocParserError::InvalidConfiguration {
                details: "Pretty print option is only supported with JSON output format".to_string(),
            });
        }

        Ok(())
    }

    /// 检查是否为批处理模式
    pub fn is_batch_mode(&self) -> bool {
        self.batch.is_some()
    }

    /// 获取批处理模式的文件列表
    pub fn get_batch_files(&self) -> Result<Vec<PathBuf>> {
        if let Some(batch_pattern) = &self.batch {
            let mut files = Vec::new();
            
            // 检查是否为目录
            let pattern_path = std::path::Path::new(batch_pattern);
            if pattern_path.is_dir() {
                // 遍历目录寻找doc/docx文件
                for entry in std::fs::read_dir(pattern_path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if let Some(ext_str) = ext.to_str() {
                                let ext_lower = ext_str.to_lowercase();
                                if ext_lower == "doc" || ext_lower == "docx" {
                                    files.push(path);
                                }
                            }
                        }
                    }
                }
            } else {
                // 作为glob模式处理
                for entry in glob::glob(batch_pattern).map_err(|e| {
                    DocParserError::InvalidConfiguration {
                        details: format!("Invalid glob pattern: {}", e),
                    }
                })? {
                    match entry {
                        Ok(path) => {
                            if path.is_file() {
                                if let Some(ext) = path.extension() {
                                    if let Some(ext_str) = ext.to_str() {
                                        let ext_lower = ext_str.to_lowercase();
                                        if ext_lower == "doc" || ext_lower == "docx" {
                                            files.push(path);
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            return Err(DocParserError::InvalidConfiguration {
                                details: format!("Glob error: {}", e),
                            });
                        }
                    }
                }
            }
            
            // 应用文件数量限制
            if let Some(max_files) = self.max_files {
                files.truncate(max_files);
            }
            
            if files.is_empty() {
                return Err(DocParserError::FileNotFound {
                    file: format!("No DOC/DOCX files found matching pattern: {}", batch_pattern),
                });
            }
            
            Ok(files)
        } else {
            Err(DocParserError::InvalidConfiguration {
                details: "Not in batch mode".to_string(),
            })
        }
    }

    /// 获取处理模式
    pub fn get_processing_mode(&self) -> ProcessingMode {
        if self.text_only {
            ProcessingMode::TextOnly
        } else if self.metadata {
            ProcessingMode::FullWithMetadata
        } else {
            ProcessingMode::Standard
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingMode {
    TextOnly,           // 只提取纯文本，最快
    Standard,           // 标准处理，包含结构
    FullWithMetadata,   // 完整处理，包含元数据
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_args_validation_unsupported_format() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let args = Args {
            input: file_path,
            output: None,
            format: "text".to_string(),
            metadata: false,
            preserve_formatting: false,
            pretty: false,
            line_numbers: false,
            text_only: false,
            verbose: false,
            batch: None,
            output_dir: None,
            overwrite: false,
            max_files: None,
        };

        let result = args.validate();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, DocParserError::UnsupportedFormat { .. }));
    }

    #[test]
    fn test_args_validation_unsupported_output_format() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.docx");
        File::create(&file_path).unwrap();

        let args = Args {
            input: file_path,
            output: None,
            format: "xml".to_string(),
            metadata: false,
            preserve_formatting: false,
            pretty: false,
            line_numbers: false,
            text_only: false,
            verbose: false,
            batch: None,
            output_dir: None,
            overwrite: false,
            max_files: None,
        };

        let result = args.validate();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, DocParserError::InvalidConfiguration { .. }));
    }

    #[test]
    fn test_args_validation_invalid_option_combination() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.docx");
        File::create(&file_path).unwrap();

        // Line numbers with non-text format
        let args = Args {
            input: file_path.clone(),
            output: None,
            format: "json".to_string(),
            metadata: false,
            preserve_formatting: false,
            pretty: false,
            line_numbers: true,
            text_only: false,
            verbose: false,
            batch: None,
            output_dir: None,
            overwrite: false,
            max_files: None,
        };

        let result = args.validate();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, DocParserError::InvalidConfiguration { .. }));

        // Pretty print with non-JSON format
        let args = Args {
            input: file_path,
            output: None,
            format: "text".to_string(),
            metadata: false,
            preserve_formatting: false,
            pretty: true,
            line_numbers: false,
            text_only: false,
            verbose: false,
            batch: None,
            output_dir: None,
            overwrite: false,
            max_files: None,
        };

        let result = args.validate();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error, DocParserError::InvalidConfiguration { .. }));
    }

    #[test]
    fn test_get_processing_mode() {
        let args = Args {
            input: PathBuf::from("test.docx"),
            output: None,
            format: "text".to_string(),
            metadata: false,
            preserve_formatting: false,
            pretty: false,
            line_numbers: false,
            text_only: true,
            verbose: false,
            batch: None,
            output_dir: None,
            overwrite: false,
            max_files: None,
        };

        assert_eq!(args.get_processing_mode(), ProcessingMode::TextOnly);

        let args = Args {
            input: PathBuf::from("test.docx"),
            output: None,
            format: "text".to_string(),
            metadata: true,
            preserve_formatting: false,
            pretty: false,
            line_numbers: false,
            text_only: false,
            verbose: false,
            batch: None,
            output_dir: None,
            overwrite: false,
            max_files: None,
        };

        assert_eq!(args.get_processing_mode(), ProcessingMode::FullWithMetadata);
    }

    #[test]
    fn test_get_output_format() {
        let args = Args {
            input: PathBuf::from("test.docx"),
            output: None,
            format: "text".to_string(),
            metadata: true,
            preserve_formatting: false,
            pretty: false,
            line_numbers: true,
            text_only: false,
            verbose: false,
            batch: None,
            output_dir: None,
            overwrite: false,
            max_files: None,
        };

        let format = args.get_output_format().unwrap();
        match format {
            crate::output::OutputFormat::Text { preserve_formatting, include_metadata, line_numbers } => {
                assert!(!preserve_formatting);
                assert!(include_metadata);
                assert!(line_numbers);
            }
            _ => panic!("Expected Text format"),
        }
    }
}