use thiserror::Error;

pub type Result<T> = std::result::Result<T, ExcelParserError>;

#[derive(Error, Debug)]
pub enum ExcelParserError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Excel parsing error: {0}")]
    Calamine(#[from] calamine::Error),
    
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Sheet not found: {0}")]
    SheetNotFound(String),
    
    #[error("Invalid range: {0}")]
    InvalidRange(String),
    
    #[error("Empty file or no data found")]
    EmptyFile,
    
    #[error("Other error: {0}")]
    Other(String),
}