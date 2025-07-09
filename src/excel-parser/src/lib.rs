pub mod parser;
pub mod output;
pub mod cli;
pub mod error;

pub use error::{ExcelParserError, Result};
pub use parser::{ExcelParser, ExcelData, Sheet};
pub use output::{OutputFormat, OutputProcessor};
pub use cli::Args;