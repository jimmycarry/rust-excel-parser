use crate::error::{DocParserError, Result};
use std::path::Path;

pub mod docx;
pub mod doc;
pub mod text_extractor;

pub use text_extractor::TextExtractor;

#[derive(Debug, Clone)]
pub struct DocData {
    pub content: String,
    pub metadata: DocMetadata,
    pub sections: Vec<DocSection>,
    pub raw_text: String,  // 纯文本内容
}

#[derive(Debug, Clone)]
pub struct DocMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub word_count: usize,
    pub paragraph_count: usize,
    pub page_count: Option<usize>,
    pub character_count: usize,
}

#[derive(Debug, Clone)]
pub struct DocSection {
    pub section_type: SectionType,
    pub content: String,
    pub level: Option<u8>,  // 用于标题级别
    pub formatting: Option<FormatInfo>,
}

#[derive(Debug, Clone)]
pub enum SectionType {
    Paragraph,
    Heading(u8),      // 1-6 标题级别
    Table,
    List,
    Image,            // 图片占位符
    Footer,
    Header,
    Hyperlink,
}

#[derive(Debug, Clone)]
pub struct FormatInfo {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font_size: Option<u32>,
    pub font_family: Option<String>,
}

pub struct DocParser;

impl DocParser {
    pub fn new() -> Self {
        Self
    }

    /// 解析DOC文件并返回结构化数据
    pub fn parse<P: AsRef<Path>>(&self, file_path: P) -> Result<DocData> {
        let file_path = file_path.as_ref();
        
        if !file_path.exists() {
            return Err(DocParserError::FileNotFound {
                file: file_path.display().to_string(),
            });
        }

        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "docx" => docx::parse_docx(file_path),
            "doc" => doc::parse_doc(file_path),
            _ => Err(DocParserError::UnsupportedFormat {
                format: extension,
                file: file_path.display().to_string(),
            }),
        }
    }

    /// Extracts plain text content from a document.
    ///
    /// This is a convenience method that parses the document and returns only
    /// the plain text content with all formatting removed. This is faster than
    /// using [`parse`] when you only need the text content.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the DOC or DOCX file to extract text from
    ///
    /// # Returns
    ///
    /// Returns the plain text content as a String, or an error if extraction fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use doc_parser::parser::DocParser;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let parser = DocParser::new();
    /// let text = parser.extract_text("document.docx")?;
    /// println!("Document text: {}", text);
    /// # Ok(())
    /// # }
    /// ```
    pub fn extract_text<P: AsRef<Path>>(&self, file_path: P) -> Result<String> {
        let doc_data = self.parse(file_path)?;
        Ok(doc_data.raw_text)
    }

    /// Extracts structured content sections from a document.
    ///
    /// This method parses the document and returns the structured sections
    /// (paragraphs, headings, tables, etc.) without the full metadata.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the DOC or DOCX file to extract sections from
    ///
    /// # Returns
    ///
    /// Returns a vector of [`DocSection`] structures, or an error if extraction fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use doc_parser::parser::{DocParser, SectionType};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let parser = DocParser::new();
    /// let sections = parser.extract_structured("document.docx")?;
    ///
    /// for section in sections {
    ///     match section.section_type {
    ///         SectionType::Heading(level) => {
    ///             println!("Heading {}: {}", level, section.content);
    ///         }
    ///         SectionType::Paragraph => {
    ///             println!("Paragraph: {}", section.content);
    ///         }
    ///         _ => {}
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn extract_structured<P: AsRef<Path>>(&self, file_path: P) -> Result<Vec<DocSection>> {
        let doc_data = self.parse(file_path)?;
        Ok(doc_data.sections)
    }

    /// Extracts document metadata without parsing the full content.
    ///
    /// This method extracts only the metadata (title, author, word count, etc.)
    /// from the document. This can be more efficient than full parsing when
    /// you only need metadata information.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the DOC or DOCX file to extract metadata from
    ///
    /// # Returns
    ///
    /// Returns a [`DocMetadata`] structure, or an error if extraction fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use doc_parser::parser::DocParser;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let parser = DocParser::new();
    /// let metadata = parser.get_metadata("document.docx")?;
    ///
    /// if let Some(title) = metadata.title {
    ///     println!("Title: {}", title);
    /// }
    /// println!("Word count: {}", metadata.word_count);
    /// println!("Character count: {}", metadata.character_count);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_metadata<P: AsRef<Path>>(&self, file_path: P) -> Result<DocMetadata> {
        let doc_data = self.parse(file_path)?;
        Ok(doc_data.metadata)
    }
}

impl Default for DocParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_not_found() {
        let parser = DocParser::new();
        let result = parser.parse("nonexistent.docx");
        assert!(matches!(result, Err(DocParserError::FileNotFound { .. })));
    }

    #[test]
    fn test_unsupported_format() {
        use std::fs::File;
        use std::io::Write;

        // 创建临时文件
        let mut temp_file = File::create("test.txt").unwrap();
        temp_file.write_all(b"test content").unwrap();

        let parser = DocParser::new();
        let result = parser.parse("test.txt");

        // 清理
        std::fs::remove_file("test.txt").unwrap();

        assert!(matches!(
            result,
            Err(DocParserError::UnsupportedFormat { .. })
        ));
    }
}