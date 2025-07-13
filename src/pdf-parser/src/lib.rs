pub mod cli;
pub mod error;
pub mod output;
pub mod parser;

pub use error::{PdfParserError, Result};
pub use output::{OutputFormat, OutputProcessor, OutputWriter};
pub use parser::{PdfData, PdfParser, Page, Table, PdfMetadata};