use crate::error::{PdfParserError, Result};
use chrono::{DateTime, Utc};
use lopdf::Document;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct PdfParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfData {
    pub pages: Vec<Page>,
    pub metadata: PdfMetadata,
    pub tables: Vec<Table>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub number: usize,
    pub text: String,
    pub tables: Vec<Table>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub page: usize,
    pub data: Vec<Vec<String>>,
    pub headers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<DateTime<Utc>>,
    pub modification_date: Option<DateTime<Utc>>,
    pub page_count: usize,
    pub file_size: u64,
}

impl PdfParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse PDF file and return structured data
    pub fn parse<P: AsRef<Path>>(&self, file_path: P) -> Result<PdfData> {
        let file_path = file_path.as_ref();

        // Check if file exists
        if !file_path.exists() {
            return Err(PdfParserError::FileNotFound(
                file_path.display().to_string(),
            ));
        }

        // Check if file has PDF extension
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        if extension != "pdf" {
            return Err(PdfParserError::UnsupportedFormat(extension));
        }

        // Get file size
        let file_size = std::fs::metadata(file_path)?.len();

        // Extract text from all pages
        let text = self.extract_text_all_pages(file_path)?;
        
        // Parse pages
        let pages = self.parse_pages(file_path, &text)?;
        
        // Extract metadata
        let mut metadata = self.extract_metadata(file_path)?;
        metadata.file_size = file_size;
        metadata.page_count = pages.len();
        
        // Extract tables from all pages
        let tables = self.extract_all_tables(&pages)?;

        Ok(PdfData {
            pages,
            metadata,
            tables,
        })
    }

    /// Parse specific page from PDF file
    pub fn parse_page<P: AsRef<Path>>(&self, file_path: P, page_number: usize) -> Result<Page> {
        let file_path = file_path.as_ref();
        
        if page_number == 0 {
            return Err(PdfParserError::InvalidPageRange(
                "Page numbers start from 1".to_string(),
            ));
        }

        let text = self.extract_text_page(file_path, page_number)?;
        let tables = self.extract_tables_from_text(&text, page_number)?;

        Ok(Page {
            number: page_number,
            text,
            tables,
        })
    }

    /// Extract only text from PDF
    pub fn extract_text<P: AsRef<Path>>(&self, file_path: P) -> Result<String> {
        self.extract_text_all_pages(file_path.as_ref())
    }

    /// Extract only tables from PDF
    pub fn extract_tables<P: AsRef<Path>>(&self, file_path: P) -> Result<Vec<Table>> {
        let text = self.extract_text_all_pages(file_path.as_ref())?;
        let pages = self.parse_pages(file_path.as_ref(), &text)?;
        self.extract_all_tables(&pages)
    }

    fn extract_text_all_pages<P: AsRef<Path>>(&self, file_path: P) -> Result<String> {
        let bytes = std::fs::read(file_path)?;
        let text = pdf_extract::extract_text_from_mem(&bytes)
            .map_err(PdfParserError::PdfExtract)?;
        
        if text.trim().is_empty() {
            return Err(PdfParserError::EmptyFile);
        }
        
        Ok(text)
    }

    fn extract_text_page<P: AsRef<Path>>(&self, file_path: P, page_number: usize) -> Result<String> {
        let doc = Document::load(file_path.as_ref())?;
        let page_ids: Vec<_> = doc.get_pages().keys().cloned().collect();
        
        if page_number > page_ids.len() {
            return Err(PdfParserError::PageNotFound(page_number));
        }
        
        // For now, extract all text and split by pages
        // This is a simplified approach - real implementation would need page-specific extraction
        let all_text = self.extract_text_all_pages(file_path)?;
        let pages: Vec<&str> = all_text.split('\x0C').collect(); // Form feed character
        
        if page_number <= pages.len() {
            Ok(pages[page_number - 1].to_string())
        } else {
            // Fallback: estimate page content
            let lines: Vec<&str> = all_text.lines().collect();
            let lines_per_page = lines.len().max(1) / page_ids.len().max(1);
            let start = (page_number - 1) * lines_per_page;
            let end = (start + lines_per_page).min(lines.len());
            
            if start < lines.len() {
                Ok(lines[start..end].join("\n"))
            } else {
                Err(PdfParserError::PageNotFound(page_number))
            }
        }
    }

    fn parse_pages<P: AsRef<Path>>(&self, file_path: P, text: &str) -> Result<Vec<Page>> {
        let doc = Document::load(file_path.as_ref())?;
        let page_count = doc.get_pages().len();
        
        // Split text by form feed or estimate pages
        let page_texts: Vec<String> = if text.contains('\x0C') {
            text.split('\x0C').map(|s| s.to_string()).collect()
        } else {
            // Estimate page breaks
            let lines: Vec<&str> = text.lines().collect();
            let lines_per_page = lines.len().max(1) / page_count.max(1);
            
            (0..page_count)
                .map(|i| {
                    let start = i * lines_per_page;
                    let end = ((i + 1) * lines_per_page).min(lines.len());
                    lines[start..end].join("\n")
                })
                .collect()
        };

        let mut pages = Vec::new();
        
        for (i, page_text) in page_texts.iter().take(page_count).enumerate() {
            let page_number = i + 1;
            let tables = self.extract_tables_from_text(page_text, page_number)?;
            
            pages.push(Page {
                number: page_number,
                text: page_text.clone(),
                tables,
            });
        }

        if pages.is_empty() {
            return Err(PdfParserError::EmptyFile);
        }

        Ok(pages)
    }

    fn extract_metadata<P: AsRef<Path>>(&self, file_path: P) -> Result<PdfMetadata> {
        let doc = Document::load(file_path.as_ref())?;
        let mut metadata = PdfMetadata {
            title: None,
            author: None,
            subject: None,
            creator: None,
            producer: None,
            creation_date: None,
            modification_date: None,
            page_count: doc.get_pages().len(),
            file_size: 0, // Will be set by caller
        };

        // Extract metadata from PDF info dictionary
        if let Ok(info_dict) = doc.trailer.get(b"Info") {
            if let Ok(info) = info_dict.as_dict() {
                // Extract title
                if let Ok(title) = info.get(b"Title") {
                    if let Ok(title_bytes) = title.as_str() {
                        if let Ok(title_str) = std::str::from_utf8(title_bytes) {
                            metadata.title = Some(title_str.to_string());
                        }
                    }
                }

                // Extract author
                if let Ok(author) = info.get(b"Author") {
                    if let Ok(author_bytes) = author.as_str() {
                        if let Ok(author_str) = std::str::from_utf8(author_bytes) {
                            metadata.author = Some(author_str.to_string());
                        }
                    }
                }

                // Extract subject
                if let Ok(subject) = info.get(b"Subject") {
                    if let Ok(subject_bytes) = subject.as_str() {
                        if let Ok(subject_str) = std::str::from_utf8(subject_bytes) {
                            metadata.subject = Some(subject_str.to_string());
                        }
                    }
                }

                // Extract creator
                if let Ok(creator) = info.get(b"Creator") {
                    if let Ok(creator_bytes) = creator.as_str() {
                        if let Ok(creator_str) = std::str::from_utf8(creator_bytes) {
                            metadata.creator = Some(creator_str.to_string());
                        }
                    }
                }

                // Extract producer
                if let Ok(producer) = info.get(b"Producer") {
                    if let Ok(producer_bytes) = producer.as_str() {
                        if let Ok(producer_str) = std::str::from_utf8(producer_bytes) {
                            metadata.producer = Some(producer_str.to_string());
                        }
                    }
                }
            }
        }

        Ok(metadata)
    }

    fn extract_tables_from_text(&self, text: &str, page: usize) -> Result<Vec<Table>> {
        let mut tables = Vec::new();
        
        // Improved table detection using multiple strategies
        let potential_tables = self.find_potential_table_blocks(text)?;
        
        for table_block in potential_tables {
            if let Ok(table_data) = self.parse_and_normalize_table(&table_block) {
                if !table_data.is_empty() && self.is_valid_table(&table_data) {
                    let headers = if self.looks_like_header(&table_data[0]) {
                        Some(table_data[0].clone())
                    } else {
                        None
                    };

                    tables.push(Table {
                        page,
                        data: table_data,
                        headers,
                    });
                }
            }
        }

        Ok(tables)
    }

    fn extract_all_tables(&self, pages: &[Page]) -> Result<Vec<Table>> {
        let mut all_tables = Vec::new();
        
        for page in pages {
            all_tables.extend(page.tables.clone());
        }
        
        Ok(all_tables)
    }

    fn find_potential_table_blocks(&self, text: &str) -> Result<Vec<Vec<String>>> {
        let mut blocks = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        
        // Look for consecutive lines that might be table rows
        let mut current_block = Vec::new();
        
        for line in lines {
            if self.could_be_table_row(line) {
                current_block.push(line.to_string());
            } else {
                // If we have accumulated enough potential table rows, save the block
                if current_block.len() >= 2 {
                    blocks.push(current_block.clone());
                }
                current_block.clear();
            }
        }
        
        // Don't forget the last block
        if current_block.len() >= 2 {
            blocks.push(current_block);
        }
        
        Ok(blocks)
    }

    fn could_be_table_row(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // Skip obvious non-table content
        if trimmed.is_empty() || trimmed.len() < 5 {
            return false;
        }
        
        // Skip lines that look like headers, paragraphs, or code
        if trimmed.starts_with('#') || // Markdown headers
           trimmed.starts_with("//") || // Comments
           trimmed.starts_with("/*") || // Comments
           trimmed.starts_with("http") || // URLs
           trimmed.contains("://") || // URLs
           trimmed.len() > 200 { // Very long lines are probably paragraphs
            return false;
        }
        
        // Look for patterns that suggest tabular structure
        let tab_count = trimmed.matches('\t').count();
        let multi_space_count = Regex::new(r"\s{3,}").unwrap().find_iter(trimmed).count();
        let pipe_count = trimmed.matches('|').count();
        
        // Could be a table row if it has multiple separators
        tab_count >= 1 || multi_space_count >= 1 || pipe_count >= 2
    }

    fn parse_and_normalize_table(&self, lines: &[String]) -> Result<Vec<Vec<String>>> {
        let mut table_data = Vec::new();
        let mut max_columns = 0;
        
        // First pass: parse all rows and find the maximum column count
        for line in lines {
            let cells = self.parse_table_row(line)?;
            if !cells.is_empty() {
                max_columns = max_columns.max(cells.len());
                table_data.push(cells);
            }
        }
        
        // Second pass: normalize all rows to have the same number of columns
        for row in &mut table_data {
            while row.len() < max_columns {
                row.push(String::new()); // Pad with empty cells
            }
            // Truncate if somehow longer (shouldn't happen)
            row.truncate(max_columns);
        }
        
        Ok(table_data)
    }

    fn parse_table_row(&self, line: &str) -> Result<Vec<String>> {
        let trimmed = line.trim();
        
        // Try different separators in order of preference
        let cells: Vec<String> = if trimmed.contains('\t') {
            // Tab-separated
            trimmed.split('\t').map(|s| s.trim().to_string()).collect()
        } else if trimmed.contains('|') && trimmed.matches('|').count() >= 2 {
            // Pipe-separated (like markdown tables)
            trimmed.split('|')
                .skip(if trimmed.starts_with('|') { 1 } else { 0 })
                .take_while(|s| !s.is_empty() || trimmed.ends_with('|'))
                .map(|s| s.trim().to_string())
                .collect()
        } else {
            // Multiple spaces
            let regex = Regex::new(r"\s{2,}")?;
            regex.split(trimmed).map(|s| s.trim().to_string()).collect()
        };
        
        // Filter out completely empty cells at the start/end, but keep internal empty cells
        let start_idx = cells.iter().position(|cell| !cell.is_empty()).unwrap_or(0);
        let end_idx = cells.iter().rposition(|cell| !cell.is_empty()).map(|i| i + 1).unwrap_or(cells.len());
        
        if start_idx < end_idx && end_idx > start_idx + 1 {
            Ok(cells[start_idx..end_idx].to_vec())
        } else {
            Ok(Vec::new())
        }
    }

    fn is_valid_table(&self, table_data: &[Vec<String>]) -> bool {
        // Table validation rules
        if table_data.len() < 2 {
            return false; // Need at least 2 rows
        }
        
        let column_count = table_data[0].len();
        if column_count < 2 {
            return false; // Need at least 2 columns
        }
        
        // Check if rows have consistent column counts (they should after normalization)
        if !table_data.iter().all(|row| row.len() == column_count) {
            return false;
        }
        
        // Check if there's meaningful content (not all cells empty)
        let non_empty_cells = table_data.iter()
            .flat_map(|row| row.iter())
            .filter(|cell| !cell.trim().is_empty())
            .count();
            
        if non_empty_cells < table_data.len() {
            return false; // Need at least one non-empty cell per row
        }
        
        true
    }

    fn looks_like_header(&self, row: &[String]) -> bool {
        // Simple heuristic: if most cells are short and don't contain numbers
        let text_cells = row.iter().filter(|cell| {
            !cell.is_empty() && 
            cell.len() < 30 && 
            !cell.chars().any(|c| c.is_ascii_digit())
        }).count();
        
        text_cells > row.len() / 2
    }

    /// Get page count from PDF file
    pub fn get_page_count<P: AsRef<Path>>(&self, file_path: P) -> Result<usize> {
        let doc = Document::load(file_path.as_ref())?;
        Ok(doc.get_pages().len())
    }
}

impl Default for PdfParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsupported_format() {
        use std::fs::File;
        use std::io::Write;

        // Create a temporary file with unsupported extension
        let mut temp_file = File::create("test.txt").unwrap();
        temp_file.write_all(b"test content").unwrap();

        let parser = PdfParser::new();
        let result = parser.parse("test.txt");

        // Clean up
        std::fs::remove_file("test.txt").unwrap();

        assert!(matches!(
            result,
            Err(PdfParserError::UnsupportedFormat(_))
        ));
    }

    #[test]
    fn test_file_not_found() {
        let parser = PdfParser::new();
        let result = parser.parse("nonexistent.pdf");
        assert!(matches!(result, Err(PdfParserError::FileNotFound(_))));
    }

    #[test]
    fn test_invalid_page_number() {
        let parser = PdfParser::new();
        let result = parser.parse_page("nonexistent.pdf", 0);
        assert!(matches!(result, Err(PdfParserError::InvalidPageRange(_))));
    }
}