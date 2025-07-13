use crate::error::{PptParserError, Result};
use chrono::{DateTime, Utc};
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zip::ZipArchive;

pub struct PptParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PptData {
    pub slides: Vec<Slide>,
    pub metadata: PptMetadata,
    pub slide_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slide {
    pub number: usize,
    pub title: Option<String>,
    pub content: Vec<String>,
    pub notes: Option<String>,
    pub tables: Vec<Table>,
    pub lists: Vec<List>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub slide_number: usize,
    pub rows: Vec<Vec<String>>,
    pub headers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub slide_number: usize,
    pub list_type: ListType,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListType {
    Ordered,
    Unordered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PptMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub creator: Option<String>,
    pub creation_date: Option<DateTime<Utc>>,
    pub modification_date: Option<DateTime<Utc>>,
    pub slide_count: usize,
    pub file_size: u64,
    pub application: Option<String>,
}

impl PptParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse PPT/PPTX file and return structured data
    pub fn parse<P: AsRef<Path>>(&self, file_path: P) -> Result<PptData> {
        let file_path = file_path.as_ref();

        // Check if file exists
        if !file_path.exists() {
            return Err(PptParserError::FileNotFound(
                file_path.display().to_string(),
            ));
        }

        // Check file extension
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "pptx" => self.parse_pptx(file_path),
            "ppt" => self.parse_ppt(file_path),
            _ => Err(PptParserError::UnsupportedFormat(extension)),
        }
    }

    /// Parse specific slide from presentation
    pub fn parse_slide<P: AsRef<Path>>(&self, file_path: P, slide_number: usize) -> Result<Slide> {
        if slide_number == 0 {
            return Err(PptParserError::InvalidSlideRange(
                "Slide numbers start from 1".to_string(),
            ));
        }

        let ppt_data = self.parse(file_path)?;
        
        ppt_data
            .slides
            .into_iter()
            .find(|slide| slide.number == slide_number)
            .ok_or(PptParserError::SlideNotFound(slide_number))
    }

    /// Get slide count from presentation
    pub fn get_slide_count<P: AsRef<Path>>(&self, file_path: P) -> Result<usize> {
        let file_path = file_path.as_ref();
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "pptx" => self.get_pptx_slide_count(file_path),
            "ppt" => Err(PptParserError::UnsupportedFormat(
                "PPT slide count extraction not supported".to_string(),
            )),
            _ => Err(PptParserError::UnsupportedFormat(extension)),
        }
    }

    fn parse_pptx<P: AsRef<Path>>(&self, file_path: P) -> Result<PptData> {
        let file = File::open(file_path.as_ref())?;
        let mut archive = ZipArchive::new(BufReader::new(file))?;
        
        // Get file size
        let file_size = std::fs::metadata(file_path.as_ref())?.len();

        // Extract metadata
        let metadata = self.extract_pptx_metadata(&mut archive, file_size)?;
        
        // Extract slides
        let slides = self.extract_pptx_slides(&mut archive)?;
        
        if slides.is_empty() {
            return Err(PptParserError::EmptyPresentation);
        }

        Ok(PptData {
            slide_count: slides.len(),
            slides,
            metadata,
        })
    }

    fn parse_ppt<P: AsRef<Path>>(&self, _file_path: P) -> Result<PptData> {
        // Basic PPT support - would need ole-rs or similar for full implementation
        Err(PptParserError::UnsupportedFormat(
            "Legacy PPT format not fully supported. Please convert to PPTX".to_string(),
        ))
    }

    fn extract_pptx_metadata(
        &self,
        archive: &mut ZipArchive<BufReader<File>>,
        file_size: u64,
    ) -> Result<PptMetadata> {
        let mut metadata = PptMetadata {
            title: None,
            author: None,
            subject: None,
            creator: None,
            creation_date: None,
            modification_date: None,
            slide_count: 0,
            file_size,
            application: None,
        };

        // Try to read core properties
        if let Ok(mut file) = archive.by_name("docProps/core.xml") {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            self.parse_core_properties(&content, &mut metadata)?;
        }

        // Try to read app properties
        if let Ok(mut file) = archive.by_name("docProps/app.xml") {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            self.parse_app_properties(&content, &mut metadata)?;
        }

        Ok(metadata)
    }

    fn parse_core_properties(&self, xml_content: &str, metadata: &mut PptMetadata) -> Result<()> {
        let mut reader = Reader::from_str(xml_content);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let mut current_element = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    current_element = String::from_utf8_lossy(name.as_ref()).to_string();
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape()?.to_string();
                    match current_element.as_str() {
                        "dc:title" => metadata.title = Some(text),
                        "dc:creator" => metadata.author = Some(text),
                        "dc:subject" => metadata.subject = Some(text),
                        "dcterms:created" => {
                            if let Ok(dt) = DateTime::parse_from_rfc3339(&text) {
                                metadata.creation_date = Some(dt.with_timezone(&Utc));
                            }
                        }
                        "dcterms:modified" => {
                            if let Ok(dt) = DateTime::parse_from_rfc3339(&text) {
                                metadata.modification_date = Some(dt.with_timezone(&Utc));
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(PptParserError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }

    fn parse_app_properties(&self, xml_content: &str, metadata: &mut PptMetadata) -> Result<()> {
        let mut reader = Reader::from_str(xml_content);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let mut current_element = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    current_element = String::from_utf8_lossy(name.as_ref()).to_string();
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape()?.to_string();
                    match current_element.as_str() {
                        "Application" => metadata.application = Some(text),
                        "Slides" => {
                            if let Ok(count) = text.parse::<usize>() {
                                metadata.slide_count = count;
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(PptParserError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }

    fn extract_pptx_slides(&self, archive: &mut ZipArchive<BufReader<File>>) -> Result<Vec<Slide>> {
        let mut slides = Vec::new();
        let mut slide_number = 1;

        // Find all slide files
        let slide_files: Vec<String> = (0..archive.len())
            .filter_map(|i| {
                if let Ok(file) = archive.by_index(i) {
                    let name = file.name().to_string();
                    if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                        Some(name)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Sort slide files to maintain order
        let mut sorted_slides = slide_files;
        sorted_slides.sort_by(|a, b| {
            let a_num = self.extract_slide_number(a).unwrap_or(0);
            let b_num = self.extract_slide_number(b).unwrap_or(0);
            a_num.cmp(&b_num)
        });

        for slide_file in sorted_slides {
            if let Ok(mut file) = archive.by_name(&slide_file) {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                
                if let Ok(slide) = self.parse_slide_xml(&content, slide_number) {
                    slides.push(slide);
                    slide_number += 1;
                }
            }
        }

        Ok(slides)
    }

    fn extract_slide_number(&self, filename: &str) -> Option<usize> {
        let re = Regex::new(r"slide(\d+)\.xml").ok()?;
        re.captures(filename)?
            .get(1)?
            .as_str()
            .parse()
            .ok()
    }

    fn parse_slide_xml(&self, xml_content: &str, slide_number: usize) -> Result<Slide> {
        let mut reader = Reader::from_str(xml_content);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();

        let mut slide = Slide {
            number: slide_number,
            title: None,
            content: Vec::new(),
            notes: None,
            tables: Vec::new(),
            lists: Vec::new(),
        };

        let mut current_text = String::new();
        let mut in_text_element = false;
        let mut text_elements = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let element_name = String::from_utf8_lossy(name.as_ref());
                    if element_name == "a:t" {
                        in_text_element = true;
                        current_text.clear();
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_text_element {
                        current_text.push_str(&e.unescape()?.to_string());
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let element_name = String::from_utf8_lossy(name.as_ref());
                    if element_name == "a:t" && in_text_element {
                        if !current_text.trim().is_empty() {
                            text_elements.push(current_text.trim().to_string());
                        }
                        in_text_element = false;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(PptParserError::XmlError(e)),
                _ => {}
            }
            buf.clear();
        }

        // Process extracted text elements
        if !text_elements.is_empty() {
            // First text element is often the title
            if text_elements.len() == 1 || self.looks_like_title(&text_elements[0]) {
                slide.title = Some(text_elements[0].clone());
                if text_elements.len() > 1 {
                    slide.content = text_elements[1..].to_vec();
                }
            } else {
                slide.content = text_elements;
            }
        }

        // Extract tables and lists
        self.extract_slide_tables_and_lists(&mut slide, xml_content)?;

        Ok(slide)
    }

    fn looks_like_title(&self, text: &str) -> bool {
        // Simple heuristic: short text, no punctuation at the end, title case
        text.len() < 100 
            && !text.ends_with('.') 
            && !text.ends_with('!') 
            && !text.ends_with('?')
            && text.chars().any(|c| c.is_uppercase())
    }

    fn extract_slide_tables_and_lists(&self, slide: &mut Slide, _xml_content: &str) -> Result<()> {
        // This is a simplified implementation
        // In a full implementation, we would parse table and list structures from the XML
        
        // Look for table-like patterns in the content
        for (i, content_item) in slide.content.iter().enumerate() {
            if self.looks_like_table_row(content_item) {
                // Try to build a table from consecutive table-like items
                let table_rows = self.extract_table_rows(&slide.content[i..]);
                if table_rows.len() > 1 {
                    let table = Table {
                        slide_number: slide.number,
                        rows: table_rows.clone(),
                        headers: if self.looks_like_header(&table_rows[0]) {
                            Some(table_rows[0].clone())
                        } else {
                            None
                        },
                    };
                    slide.tables.push(table);
                }
            }
        }

        // Look for list patterns
        for content_item in &slide.content {
            if self.looks_like_list_item(content_item) {
                let list_items = self.extract_list_items(&slide.content);
                if !list_items.is_empty() {
                    let list = List {
                        slide_number: slide.number,
                        list_type: if content_item.trim_start().starts_with(char::is_numeric) {
                            ListType::Ordered
                        } else {
                            ListType::Unordered
                        },
                        items: list_items,
                    };
                    slide.lists.push(list);
                    break; // Only extract one list per slide for now
                }
            }
        }

        Ok(())
    }

    fn looks_like_table_row(&self, text: &str) -> bool {
        // Simple heuristic: contains multiple tab-separated or pipe-separated values
        text.contains('\t') || text.matches('|').count() >= 2 || text.matches("  ").count() >= 2
    }

    fn looks_like_header(&self, row: &[String]) -> bool {
        // Simple heuristic: all items are short and don't contain numbers
        row.iter().all(|cell| {
            cell.len() < 30 && !cell.chars().any(|c| c.is_ascii_digit())
        })
    }

    fn extract_table_rows(&self, content: &[String]) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        
        for item in content {
            if !self.looks_like_table_row(item) {
                break;
            }
            
            let cells: Vec<String> = if item.contains('\t') {
                item.split('\t').map(|s| s.trim().to_string()).collect()
            } else if item.contains('|') {
                item.split('|')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                // Split on multiple spaces
                let re = Regex::new(r"\s{2,}").unwrap();
                re.split(item).map(|s| s.trim().to_string()).collect()
            };
            
            if cells.len() > 1 {
                rows.push(cells);
            }
        }
        
        rows
    }

    fn looks_like_list_item(&self, text: &str) -> bool {
        let trimmed = text.trim_start();
        trimmed.starts_with("• ") 
            || trimmed.starts_with("- ") 
            || trimmed.starts_with("* ") 
            || (trimmed.len() > 2 && trimmed.chars().next().unwrap().is_ascii_digit() && trimmed.chars().nth(1) == Some('.'))
    }

    fn extract_list_items(&self, content: &[String]) -> Vec<String> {
        content
            .iter()
            .filter(|item| self.looks_like_list_item(item))
            .map(|item| {
                let trimmed = item.trim_start();
                if trimmed.starts_with("• ") || trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                    trimmed[2..].to_string()
                } else if trimmed.len() > 2 && trimmed.chars().next().unwrap().is_ascii_digit() {
                    // Skip number and dot
                    trimmed.splitn(2, '.').nth(1).unwrap_or("").trim().to_string()
                } else {
                    trimmed.to_string()
                }
            })
            .filter(|item| !item.is_empty())
            .collect()
    }

    fn get_pptx_slide_count<P: AsRef<Path>>(&self, file_path: P) -> Result<usize> {
        let file = File::open(file_path)?;
        let mut archive = ZipArchive::new(BufReader::new(file))?;

        let slide_count = (0..archive.len())
            .filter(|&i| {
                if let Ok(file) = archive.by_index(i) {
                    let name = file.name();
                    name.starts_with("ppt/slides/slide") && name.ends_with(".xml")
                } else {
                    false
                }
            })
            .count();

        Ok(slide_count)
    }
}

impl Default for PptParser {
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

        let parser = PptParser::new();
        let result = parser.parse("test.txt");

        // Clean up
        std::fs::remove_file("test.txt").unwrap();

        assert!(matches!(
            result,
            Err(PptParserError::UnsupportedFormat(_))
        ));
    }

    #[test]
    fn test_file_not_found() {
        let parser = PptParser::new();
        let result = parser.parse("nonexistent.pptx");
        assert!(matches!(result, Err(PptParserError::FileNotFound(_))));
    }

    #[test]
    fn test_invalid_slide_number() {
        let parser = PptParser::new();
        let result = parser.parse_slide("nonexistent.pptx", 0);
        assert!(matches!(result, Err(PptParserError::InvalidSlideRange(_))));
    }

    #[test]
    fn test_extract_slide_number() {
        let parser = PptParser::new();
        assert_eq!(parser.extract_slide_number("ppt/slides/slide1.xml"), Some(1));
        assert_eq!(parser.extract_slide_number("ppt/slides/slide42.xml"), Some(42));
        assert_eq!(parser.extract_slide_number("other.xml"), None);
    }

    #[test]
    fn test_looks_like_title() {
        let parser = PptParser::new();
        assert!(parser.looks_like_title("Project Overview"));
        assert!(parser.looks_like_title("Introduction to Rust"));
        assert!(!parser.looks_like_title("This is a very long sentence that looks more like content than a title."));
        assert!(!parser.looks_like_title("What is this?"));
    }

    #[test]
    fn test_looks_like_table_row() {
        let parser = PptParser::new();
        assert!(parser.looks_like_table_row("Name\tAge\tCity"));
        assert!(parser.looks_like_table_row("John | 30 | New York"));
        assert!(parser.looks_like_table_row("Item  Price  Quantity"));
        assert!(!parser.looks_like_table_row("Simple text"));
    }

    #[test]
    fn test_looks_like_list_item() {
        let parser = PptParser::new();
        assert!(parser.looks_like_list_item("• First item"));
        assert!(parser.looks_like_list_item("- Second item"));
        assert!(parser.looks_like_list_item("* Third item"));
        assert!(parser.looks_like_list_item("1. Numbered item"));
        assert!(!parser.looks_like_list_item("Regular text"));
    }
}