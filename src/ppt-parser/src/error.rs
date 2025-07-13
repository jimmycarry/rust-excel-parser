use thiserror::Error;

pub type Result<T> = std::result::Result<T, PptParserError>;

#[derive(Error, Debug)]
pub enum PptParserError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("Slide not found: {0}")]
    SlideNotFound(usize),

    #[error("Invalid slide range: {0}")]
    InvalidSlideRange(String),

    #[error("Empty presentation")]
    EmptyPresentation,

    #[error("ZIP extraction error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("XML parsing error: {0}")]
    XmlError(#[from] quick_xml::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Invalid XML structure: {0}")]
    InvalidXmlStructure(String),

    #[error("Presentation parsing error: {0}")]
    ParsingError(String),

    #[error("Metadata extraction error: {0}")]
    MetadataError(String),

    #[error("Content extraction error: {0}")]
    ContentError(String),
}

impl PptParserError {
    pub fn parsing_error(msg: impl Into<String>) -> Self {
        Self::ParsingError(msg.into())
    }

    pub fn metadata_error(msg: impl Into<String>) -> Self {
        Self::MetadataError(msg.into())
    }

    pub fn content_error(msg: impl Into<String>) -> Self {
        Self::ContentError(msg.into())
    }

    pub fn invalid_xml(msg: impl Into<String>) -> Self {
        Self::InvalidXmlStructure(msg.into())
    }
}