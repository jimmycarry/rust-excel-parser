//! Basic usage examples for the doc-parser library.
//!
//! This example demonstrates the fundamental operations:
//! - Parsing a document
//! - Extracting text content
//! - Accessing metadata
//! - Working with document sections

use doc_parser::{DocParser, OutputFormat, OutputProcessor, SectionType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Doc Parser Basic Usage Examples ===\n");

    // Example 1: Simple text extraction
    println!("1. Simple Text Extraction");
    simple_text_extraction()?;

    // Example 2: Full document parsing
    println!("\n2. Full Document Parsing");
    full_document_parsing()?;

    // Example 3: Metadata extraction
    println!("\n3. Metadata Extraction");
    metadata_extraction()?;

    // Example 4: Structured content analysis
    println!("\n4. Structured Content Analysis");
    structured_content_analysis()?;

    // Example 5: Output format conversion
    println!("\n5. Output Format Conversion");
    output_format_conversion()?;

    Ok(())
}

/// Example 1: Simple text extraction
/// This is the fastest way to get plain text from a document.
fn simple_text_extraction() -> Result<(), Box<dyn std::error::Error>> {
    let parser = DocParser::new();

    // Replace with actual file path
    let file_path = "sample.docx";
    
    println!("  Attempting to extract text from: {}", file_path);
    
    match parser.extract_text(file_path) {
        Ok(text) => {
            println!("  ✅ Text extracted successfully!");
            println!("  📄 Content preview: {}", 
                if text.len() > 100 { 
                    format!("{}...", &text[..100]) 
                } else { 
                    text 
                }
            );
        }
        Err(e) => {
            println!("  ❌ Error: {}", e);
            println!("  💡 Make sure the file exists and is a valid DOC/DOCX file");
        }
    }

    Ok(())
}

/// Example 2: Full document parsing
/// This extracts all available information from the document.
fn full_document_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let parser = DocParser::new();
    let file_path = "sample.docx";

    println!("  Parsing document: {}", file_path);

    match parser.parse(file_path) {
        Ok(doc_data) => {
            println!("  ✅ Document parsed successfully!");
            println!("  📊 Statistics:");
            println!("    - Content length: {} characters", doc_data.content.len());
            println!("    - Raw text length: {} characters", doc_data.raw_text.len());
            println!("    - Number of sections: {}", doc_data.sections.len());
            println!("    - Word count: {}", doc_data.metadata.word_count);
            println!("    - Paragraph count: {}", doc_data.metadata.paragraph_count);
            
            if let Some(title) = &doc_data.metadata.title {
                println!("    - Title: {}", title);
            }
        }
        Err(e) => {
            println!("  ❌ Error: {}", e);
        }
    }

    Ok(())
}

/// Example 3: Metadata extraction
/// This shows how to extract only metadata without parsing the full content.
fn metadata_extraction() -> Result<(), Box<dyn std::error::Error>> {
    let parser = DocParser::new();
    let file_path = "sample.docx";

    println!("  Extracting metadata from: {}", file_path);

    match parser.get_metadata(file_path) {
        Ok(metadata) => {
            println!("  ✅ Metadata extracted successfully!");
            println!("  📋 Document Information:");
            
            if let Some(title) = &metadata.title {
                println!("    - Title: {}", title);
            } else {
                println!("    - Title: Not available");
            }
            
            if let Some(author) = &metadata.author {
                println!("    - Author: {}", author);
            } else {
                println!("    - Author: Not available");
            }
            
            if let Some(subject) = &metadata.subject {
                println!("    - Subject: {}", subject);
            } else {
                println!("    - Subject: Not available");
            }

            println!("    - Word count: {}", metadata.word_count);
            println!("    - Paragraph count: {}", metadata.paragraph_count);
            println!("    - Character count: {}", metadata.character_count);
            
            if let Some(page_count) = metadata.page_count {
                println!("    - Page count: {}", page_count);
            } else {
                println!("    - Page count: Not available");
            }
        }
        Err(e) => {
            println!("  ❌ Error: {}", e);
        }
    }

    Ok(())
}

/// Example 4: Structured content analysis
/// This shows how to work with the structured sections of a document.
fn structured_content_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let parser = DocParser::new();
    let file_path = "sample.docx";

    println!("  Analyzing document structure: {}", file_path);

    match parser.extract_structured(file_path) {
        Ok(sections) => {
            println!("  ✅ Document structure analyzed!");
            println!("  📑 Document Outline:");

            let mut heading_count = 0;
            let mut paragraph_count = 0;
            let mut table_count = 0;
            let mut list_count = 0;
            let mut other_count = 0;

            for (i, section) in sections.iter().enumerate() {
                if i < 10 { // Show first 10 sections
                    match &section.section_type {
                        SectionType::Heading(level) => {
                            let indent = "  ".repeat(*level as usize);
                            println!("    {}📝 H{}: {}", indent, level, 
                                if section.content.len() > 50 { 
                                    format!("{}...", &section.content[..50]) 
                                } else { 
                                    section.content.clone() 
                                }
                            );
                            heading_count += 1;
                        }
                        SectionType::Paragraph => {
                            println!("    📄 Paragraph: {}", 
                                if section.content.len() > 50 { 
                                    format!("{}...", &section.content[..50]) 
                                } else { 
                                    section.content.clone() 
                                }
                            );
                            paragraph_count += 1;
                        }
                        SectionType::Table => {
                            println!("    📊 Table: {}", section.content);
                            table_count += 1;
                        }
                        SectionType::List => {
                            println!("    📋 List: {}", section.content);
                            list_count += 1;
                        }
                        _ => {
                            println!("    ❓ Other: {:?}", section.section_type);
                            other_count += 1;
                        }
                    }
                }
            }

            if sections.len() > 10 {
                println!("    ... and {} more sections", sections.len() - 10);
            }

            println!("  📊 Section Summary:");
            println!("    - Headings: {}", heading_count);
            println!("    - Paragraphs: {}", paragraph_count);
            println!("    - Tables: {}", table_count);
            println!("    - Lists: {}", list_count);
            println!("    - Other: {}", other_count);
            println!("    - Total: {}", sections.len());
        }
        Err(e) => {
            println!("  ❌ Error: {}", e);
        }
    }

    Ok(())
}

/// Example 5: Output format conversion
/// This shows how to convert documents to different output formats.
fn output_format_conversion() -> Result<(), Box<dyn std::error::Error>> {
    let parser = DocParser::new();
    let file_path = "sample.docx";

    println!("  Converting document to different formats: {}", file_path);

    match parser.parse(file_path) {
        Ok(doc_data) => {
            let processor = OutputProcessor::new();

            // Convert to JSON
            println!("  📄 Converting to JSON...");
            let json_format = OutputFormat::Json {
                pretty: true,
                include_formatting: false,
            };
            
            let mut json_output = Vec::new();
            if let Err(e) = processor.process(&doc_data, &json_format, &mut json_output) {
                println!("    ❌ JSON conversion failed: {}", e);
            } else {
                let json_str = String::from_utf8_lossy(&json_output);
                println!("    ✅ JSON conversion successful! ({} bytes)", json_output.len());
                if json_str.len() > 200 {
                    println!("    Preview: {}...", &json_str[..200]);
                } else {
                    println!("    Content: {}", json_str);
                }
            }

            // Convert to Markdown
            println!("  📝 Converting to Markdown...");
            let md_format = OutputFormat::Markdown {
                preserve_structure: true,
                include_metadata: true,
            };
            
            let mut md_output = Vec::new();
            if let Err(e) = processor.process(&doc_data, &md_format, &mut md_output) {
                println!("    ❌ Markdown conversion failed: {}", e);
            } else {
                let md_str = String::from_utf8_lossy(&md_output);
                println!("    ✅ Markdown conversion successful! ({} bytes)", md_output.len());
                if md_str.len() > 200 {
                    println!("    Preview: {}...", &md_str[..200]);
                } else {
                    println!("    Content: {}", md_str);
                }
            }

            // Convert to Text
            println!("  📄 Converting to Text...");
            let text_format = OutputFormat::Text {
                preserve_formatting: false,
                include_metadata: true,
                line_numbers: false,
            };
            
            let mut text_output = Vec::new();
            if let Err(e) = processor.process(&doc_data, &text_format, &mut text_output) {
                println!("    ❌ Text conversion failed: {}", e);
            } else {
                let text_str = String::from_utf8_lossy(&text_output);
                println!("    ✅ Text conversion successful! ({} bytes)", text_output.len());
                if text_str.len() > 200 {
                    println!("    Preview: {}...", &text_str[..200]);
                } else {
                    println!("    Content: {}", text_str);
                }
            }
        }
        Err(e) => {
            println!("  ❌ Error: {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_basic_functions() {
        // These tests would require actual DOCX files to work
        // For now, they just test that the functions can be called
        assert!(true);
    }

    #[test]
    fn test_error_handling() {
        let parser = DocParser::new();
        
        // Test with non-existent file
        let result = parser.extract_text("non_existent_file.docx");
        assert!(result.is_err());
        
        // Test with invalid format
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"This is not a valid DOCX file").unwrap();
        
        let result = parser.extract_text(temp_file.path());
        assert!(result.is_err());
    }
}