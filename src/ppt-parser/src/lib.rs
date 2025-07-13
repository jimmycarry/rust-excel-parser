//! # PPT Parser
//!
//! A cross-platform PowerPoint parser written in Rust that converts PPT and PPTX files
//! to multiple text formats including plain text, JSON, Markdown, and HTML.
//!
//! ## Features
//!
//! - **Multiple Input Formats**: Supports .pptx (primary) and .ppt (basic support)
//! - **Multiple Output Formats**: Text, JSON, Markdown, HTML
//! - **Metadata Extraction**: Title, author, creation date, slide count, etc.
//! - **Structured Content**: Extracts text, tables, lists, and speaker notes
//! - **Slide-level Processing**: Process individual slides or entire presentations
//! - **Memory Efficient**: Streaming architecture for large presentations
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ppt_parser::{PptParser, OutputFormat, OutputProcessor};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let parser = PptParser::new();
//! let ppt_data = parser.parse("presentation.pptx")?;
//!
//! let format = OutputFormat::json_pretty_with_metadata();
//! let processor = OutputProcessor::new();
//! processor.process(&ppt_data, &format, &mut std::io::stdout())?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Output Formats
//!
//! ### Text
//! Plain text extraction with slide organization:
//! ```text
//! Presentation: My Presentation
//! Author: John Doe
//! Slides: 5
//!
//! SLIDE 1
//! Title: Introduction
//! Content:
//!   Welcome to our presentation
//!   Key topics covered today
//! ```
//!
//! ### JSON
//! Structured data format:
//! ```json
//! {
//!   "metadata": {
//!     "title": "My Presentation",
//!     "author": "John Doe",
//!     "slide_count": 5
//!   },
//!   "slides": [
//!     {
//!       "number": 1,
//!       "title": "Introduction",
//!       "content": ["Welcome to our presentation"],
//!       "tables": [],
//!       "lists": []
//!     }
//!   ]
//! }
//! ```
//!
//! ### Markdown
//! Markdown format with YAML frontmatter:
//! ```markdown
//! ---
//! title: "My Presentation"
//! author: "John Doe"
//! slide_count: 5
//! ---
//!
//! # My Presentation
//!
//! ## Slide 1: Introduction
//!
//! Welcome to our presentation
//! ```
//!
//! ### HTML
//! Complete HTML document with CSS styling:
//! ```html
//! <!DOCTYPE html>
//! <html>
//! <head>
//!     <title>My Presentation</title>
//!     <style>/* CSS styles */</style>
//! </head>
//! <body>
//!     <div class="presentation">
//!         <h1>My Presentation</h1>
//!         <div class="slide">
//!             <h2>Introduction</h2>
//!             <p>Welcome to our presentation</p>
//!         </div>
//!     </div>
//! </body>
//! </html>
//! ```

pub mod cli;
pub mod error;
pub mod output;
pub mod parser;

pub use cli::Args;
pub use error::{PptParserError, Result};
pub use output::{OutputFormat, OutputProcessor, OutputWriter};
pub use parser::{
    List, ListType, PptData, PptMetadata, PptParser, Slide, Table,
};

/// Convenience function to parse a PPT/PPTX file and return structured data
///
/// # Arguments
///
/// * `file_path` - Path to the PPT/PPTX file
///
/// # Returns
///
/// Returns `PptData` containing slides, metadata, and structured content
///
/// # Example
///
/// ```rust,no_run
/// use ppt_parser::parse_file;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let data = parse_file("presentation.pptx")?;
/// println!("Parsed {} slides", data.slide_count);
/// # Ok(())
/// # }
/// ```
pub fn parse_file<P: AsRef<std::path::Path>>(file_path: P) -> Result<PptData> {
    let parser = PptParser::new();
    parser.parse(file_path)
}

/// Convenience function to parse a specific slide from a PPT/PPTX file
///
/// # Arguments
///
/// * `file_path` - Path to the PPT/PPTX file
/// * `slide_number` - Slide number (1-based)
///
/// # Returns
///
/// Returns `Slide` containing the specific slide's content
///
/// # Example
///
/// ```rust,no_run
/// use ppt_parser::parse_slide;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let slide = parse_slide("presentation.pptx", 1)?;
/// println!("Slide title: {:?}", slide.title);
/// # Ok(())
/// # }
/// ```
pub fn parse_slide<P: AsRef<std::path::Path>>(
    file_path: P,
    slide_number: usize,
) -> Result<Slide> {
    let parser = PptParser::new();
    parser.parse_slide(file_path, slide_number)
}

/// Convenience function to convert PPT/PPTX data to a specific format
///
/// # Arguments
///
/// * `data` - The parsed PPT data
/// * `format` - The desired output format
/// * `writer` - Where to write the output
///
/// # Example
///
/// ```rust,no_run
/// use ppt_parser::{parse_file, convert_to_format, OutputFormat};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let data = parse_file("presentation.pptx")?;
/// let format = OutputFormat::json_pretty_with_metadata();
/// convert_to_format(&data, &format, &mut std::io::stdout())?;
/// # Ok(())
/// # }
/// ```
pub fn convert_to_format<W: std::io::Write>(
    data: &PptData,
    format: &OutputFormat,
    writer: &mut W,
) -> Result<()> {
    let processor = OutputProcessor::new();
    processor.process(data, format, writer)
}

/// Convenience function to convert a single slide to a specific format
///
/// # Arguments
///
/// * `slide` - The slide to convert
/// * `format` - The desired output format
/// * `writer` - Where to write the output
///
/// # Example
///
/// ```rust,no_run
/// use ppt_parser::{parse_slide, convert_slide_to_format, OutputFormat};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let slide = parse_slide("presentation.pptx", 1)?;
/// let format = OutputFormat::markdown();
/// convert_slide_to_format(&slide, &format, &mut std::io::stdout())?;
/// # Ok(())
/// # }
/// ```
pub fn convert_slide_to_format<W: std::io::Write>(
    slide: &Slide,
    format: &OutputFormat,
    writer: &mut W,
) -> Result<()> {
    let processor = OutputProcessor::new();
    processor.process_slide(slide, format, writer)
}