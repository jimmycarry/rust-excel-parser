//! # Doc Parser Library
//!
//! A high-performance, cross-platform document parser for Microsoft Word documents (DOC/DOCX)
//! written in Rust. Extract text content, metadata, and structured data from Word documents
//! with support for multiple output formats.
//!
//! ## Features
//!
//! - 🚀 **High Performance**: Built with Rust for speed and memory efficiency
//! - 📄 **Multiple Formats**: Support for both legacy DOC and modern DOCX files
//! - 🎯 **Flexible Output**: Export to Text, Markdown, and JSON formats
//! - 🔄 **Batch Processing**: Process multiple files at once with directory scanning
//! - 📊 **Rich Metadata**: Extract document properties, word counts, and structure
//! - 🛠️ **CLI & Library**: Use as a command-line tool or integrate as a Rust library
//! - 🌍 **Cross-Platform**: Works on Windows, macOS, and Linux
//! - 💪 **Robust Error Handling**: User-friendly error messages with helpful suggestions
//!
//! ## Quick Start
//!
//! ### Basic Text Extraction
//!
//! ```rust,no_run
//! use doc_parser::DocParser;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let parser = DocParser::new();
//! let text = parser.extract_text("document.docx")?;
//! println!("Extracted text: {}", text);
//! # Ok(())
//! # }
//! ```
//!
//! ### Structured Document Parsing
//!
//! ```rust,no_run
//! use doc_parser::{DocParser, OutputFormat, OutputProcessor};
//! use std::io::Cursor;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let parser = DocParser::new();
//! let doc_data = parser.parse("document.docx")?;
//!
//! // Access metadata
//! println!("Title: {:?}", doc_data.metadata.title);
//! println!("Word count: {}", doc_data.metadata.word_count);
//!
//! // Convert to JSON
//! let format = OutputFormat::Json {
//!     pretty: true,
//!     include_formatting: false,
//! };
//!
//! let processor = OutputProcessor::new();
//! let mut output = Vec::new();
//! processor.process(&doc_data, &format, &mut output)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Batch Processing
//!
//! ```rust,no_run
//! use doc_parser::{Args, DocParser, OutputProcessor};
//! use std::path::PathBuf;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create Args structure for batch processing
//! let mut args = Args {
//!     input: PathBuf::new(), // Not used in batch mode
//!     output: None,
//!     format: "json".to_string(),
//!     metadata: true,
//!     preserve_formatting: false,
//!     pretty: true,
//!     line_numbers: false,
//!     text_only: false,
//!     verbose: true,
//!     batch: Some("./documents".to_string()),
//!     output_dir: Some(PathBuf::from("./output")),
//!     overwrite: false,
//!     max_files: Some(10),
//! };
//!
//! if args.is_batch_mode() {
//!     let files = args.get_batch_files()?;
//!     println!("Found {} files to process", files.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Module Organization
//!
//! - [`parser`] - Core document parsing functionality
//! - [`output`] - Output format processors (Text, Markdown, JSON)
//! - [`cli`] - Command-line interface and argument parsing
//! - [`error`] - Error types and handling
//!
//! ## Processing Modes
//!
//! The parser supports different processing modes for optimal performance:
//!
//! - **Fast Mode**: Extract only plain text (`--text-only`)
//! - **Standard Mode**: Extract text with basic structure
//! - **Full Mode**: Extract everything including metadata (`--metadata`)
//!
//! ## Supported File Formats
//!
//! - **DOCX**: Full support (Office 2007+)
//! - **DOC**: Basic support (requires `legacy-doc` feature)
//!
//! ## Error Handling
//!
//! All parsing operations return a [`Result`] type with detailed error information.
//! Errors implement user-friendly messages with actionable suggestions.
//!
//! ```rust
//! use doc_parser::{DocParser, DocParserError};
//!
//! # fn main() {
//! let parser = DocParser::new();
//! match parser.parse("document.docx") {
//!     Ok(doc_data) => {
//!         // Process document
//!     }
//!     Err(DocParserError::FileNotFound { file }) => {
//!         eprintln!("File not found: {}", file);
//!     }
//!     Err(DocParserError::UnsupportedFormat { format, .. }) => {
//!         eprintln!("Unsupported format: {}", format);
//!     }
//!     Err(e) => {
//!         eprintln!("Error: {}", e.user_friendly_message());
//!     }
//! }
//! # }
//! ```

pub mod parser;
pub mod output;
pub mod cli;
pub mod error;

// Re-export commonly used types for convenience
pub use error::{DocParserError, Result};
pub use parser::{DocParser, DocData, DocSection, DocMetadata, SectionType, FormatInfo};
pub use output::{OutputFormat, OutputProcessor};
pub use cli::{Args, ProcessingMode};

// Re-export the main parser for convenience
pub use parser::DocParser as Parser;