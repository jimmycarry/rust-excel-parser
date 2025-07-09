use thiserror::Error;

pub type Result<T> = std::result::Result<T, DocParserError>;

#[derive(Error, Debug)]
pub enum DocParserError {
    #[error("Failed to read file '{file}': {source}")]
    IoError {
        file: String,
        #[source]
        source: std::io::Error,
    },
    
    #[error("The file '{file}' is not a valid DOC file or is corrupted.\nHelp: Please check if the file exists and is a valid Microsoft Word document.")]
    DocParsing {
        file: String,
        details: String,
    },
    
    #[error("The file '{file}' is not a valid DOCX file or is corrupted.\nHelp: Please check if the file exists and is a valid Microsoft Word document (Office 2007+).")]
    DocxParsing {
        file: String,
        details: String,
    },
    
    #[error("Failed to serialize output to JSON format: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Failed to extract text from document '{file}': {details}")]
    TextExtraction {
        file: String,
        details: String,
    },
    
    #[error("File not found: '{file}'.\nHelp: Please check the file path and ensure the file exists.")]
    FileNotFound {
        file: String,
    },
    
    #[error("Unsupported file format: '.{format}'.\nHelp: Supported formats are: .doc, .docx.\nIf you have a different format, please convert it to Word format first.")]
    UnsupportedFormat {
        format: String,
        file: String,
    },
    
    #[error("The file '{file}' appears to be empty or contains no readable content.\nHelp: Please check if the file is corrupted or try opening it in Microsoft Word.")]
    EmptyFile {
        file: String,
    },
    
    #[error("Invalid document structure in '{file}': {details}")]
    InvalidStructure {
        file: String,
        details: String,
    },
    
    #[error("Text encoding error in '{file}': {details}")]
    Encoding {
        file: String,
        details: String,
    },
    
    #[error("Output directory does not exist: '{directory}'.\nHelp: Please create the directory first or use a different output path.")]
    OutputDirectoryNotFound {
        directory: String,
    },
    
    #[error("Permission denied: {details}.\nHelp: Please check file permissions and try again.")]
    PermissionDenied {
        details: String,
    },
    
    #[error("Invalid configuration: {details}")]
    InvalidConfiguration {
        details: String,
    },
    
    #[error("Unexpected error: {0}")]
    Other(String),
}

impl DocParserError {
    /// 创建用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            DocParserError::FileNotFound { file } => {
                format!("❌ File not found: '{file}'\n💡 Make sure the file path is correct and the file exists.")
            }
            DocParserError::UnsupportedFormat { format, .. } => {
                format!("❌ Unsupported file format: '.{format}'\n💡 Supported formats: .doc, .docx\n💡 Try converting your file to Word format first.")
            }
            DocParserError::DocxParsing { file, .. } => {
                format!("❌ Failed to parse DOCX file: '{file}'\n💡 The file might be corrupted or not a valid DOCX file.\n💡 Try opening it in Microsoft Word to verify it's valid.")
            }
            DocParserError::DocParsing { file, .. } => {
                format!("❌ Failed to parse DOC file: '{file}'\n💡 The file might be corrupted or not a valid DOC file.\n💡 Try opening it in Microsoft Word to verify it's valid.")
            }
            DocParserError::EmptyFile { file } => {
                format!("❌ File appears to be empty: '{file}'\n💡 Check if the file contains any content.")
            }
            DocParserError::OutputDirectoryNotFound { directory } => {
                format!("❌ Output directory doesn't exist: '{directory}'\n💡 Create the directory first: mkdir -p '{directory}'")
            }
            DocParserError::PermissionDenied { details } => {
                format!("❌ Permission denied: {details}\n💡 Check file permissions or try running with appropriate privileges.")
            }
            _ => self.to_string(),
        }
    }
}

// 从std::io::Error转换为DocParserError
impl From<std::io::Error> for DocParserError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => DocParserError::FileNotFound { 
                file: "unknown".to_string() 
            },
            std::io::ErrorKind::PermissionDenied => DocParserError::PermissionDenied { 
                details: error.to_string() 
            },
            _ => DocParserError::IoError { 
                file: "unknown".to_string(), 
                source: error 
            },
        }
    }
}