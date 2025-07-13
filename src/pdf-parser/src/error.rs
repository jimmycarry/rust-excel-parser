use thiserror::Error;

pub type Result<T> = std::result::Result<T, PdfParserError>;

#[derive(Error, Debug)]
pub enum PdfParserError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("PDF parsing error: {0}")]
    PdfExtract(#[from] pdf_extract::OutputError),
    
    #[error("PDF document error: {0}")]
    Lopdf(#[from] lopdf::Error),
    
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Page not found: {0}")]
    PageNotFound(usize),
    
    #[error("Invalid page range: {0}")]
    InvalidPageRange(String),
    
    #[error("Empty file or no data found")]
    EmptyFile,
    
    #[error("Password protected PDF: {0}")]
    PasswordProtected(String),
    
    #[error("Corrupted PDF file: {0}")]
    CorruptedFile(String),
    
    #[error("Table extraction failed: {0}")]
    TableExtractionFailed(String),
    
    #[error("Other error: {0}")]
    Other(String),
}