use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{PptData, Slide};
use serde_json;
use std::io::Write;

pub struct JsonOutput {
    pretty: bool,
    include_metadata: bool,
}

impl JsonOutput {
    pub fn new(pretty: bool, include_metadata: bool) -> Self {
        Self {
            pretty,
            include_metadata,
        }
    }
}

impl OutputWriter for JsonOutput {
    fn write_ppt_data<W: Write>(&self, data: &PptData, writer: &mut W) -> Result<()> {
        let json_data = if self.include_metadata {
            serde_json::json!({
                "metadata": data.metadata,
                "slide_count": data.slide_count,
                "slides": data.slides
            })
        } else {
            serde_json::json!({
                "slide_count": data.slide_count,
                "slides": data.slides
            })
        };

        if self.pretty {
            let formatted = serde_json::to_string_pretty(&json_data)?;
            write!(writer, "{}", formatted)?;
        } else {
            let formatted = serde_json::to_string(&json_data)?;
            write!(writer, "{}", formatted)?;
        }

        Ok(())
    }

    fn write_slide<W: Write>(&self, slide: &Slide, writer: &mut W) -> Result<()> {
        if self.pretty {
            let formatted = serde_json::to_string_pretty(slide)?;
            write!(writer, "{}", formatted)?;
        } else {
            let formatted = serde_json::to_string(slide)?;
            write!(writer, "{}", formatted)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{List, ListType, PptMetadata, Table};
    use chrono::Utc;

    #[test]
    fn test_json_slide_output() {
        let slide = Slide {
            number: 1,
            title: Some("Test Slide".to_string()),
            content: vec!["First point".to_string(), "Second point".to_string()],
            notes: Some("Important notes".to_string()),
            tables: vec![],
            lists: vec![],
        };

        let output = JsonOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert_eq!(parsed["number"], 1);
        assert_eq!(parsed["title"], "Test Slide");
        assert_eq!(parsed["content"][0], "First point");
        assert_eq!(parsed["content"][1], "Second point");
        assert_eq!(parsed["notes"], "Important notes");
    }

    #[test]
    fn test_json_slide_with_table() {
        let table = Table {
            slide_number: 1,
            rows: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "30".to_string()],
            ],
            headers: Some(vec!["Name".to_string(), "Age".to_string()]),
        };

        let slide = Slide {
            number: 1,
            title: None,
            content: vec![],
            notes: None,
            tables: vec![table],
            lists: vec![],
        };

        let output = JsonOutput::new(true, false);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert!(parsed["tables"].is_array());
        assert_eq!(parsed["tables"][0]["rows"][0][0], "Name");
        assert_eq!(parsed["tables"][0]["headers"][0], "Name");
    }

    #[test]
    fn test_json_slide_with_list() {
        let list = List {
            slide_number: 1,
            list_type: ListType::Ordered,
            items: vec!["First".to_string(), "Second".to_string()],
        };

        let slide = Slide {
            number: 1,
            title: None,
            content: vec![],
            notes: None,
            tables: vec![],
            lists: vec![list],
        };

        let output = JsonOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert!(parsed["lists"].is_array());
        assert_eq!(parsed["lists"][0]["list_type"], "Ordered");
        assert_eq!(parsed["lists"][0]["items"][0], "First");
    }

    #[test]
    fn test_json_ppt_data_with_metadata() {
        let metadata = PptMetadata {
            title: Some("Test Presentation".to_string()),
            author: Some("Test Author".to_string()),
            subject: None,
            creator: None,
            creation_date: Some(Utc::now()),
            modification_date: None,
            slide_count: 1,
            file_size: 1024,
            application: None,
        };

        let slide = Slide {
            number: 1,
            title: Some("Slide 1".to_string()),
            content: vec!["Content".to_string()],
            notes: None,
            tables: vec![],
            lists: vec![],
        };

        let ppt_data = PptData {
            slides: vec![slide],
            metadata,
            slide_count: 1,
        };

        let output = JsonOutput::new(true, true);
        let mut buffer = Vec::new();
        output.write_ppt_data(&ppt_data, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert!(parsed["metadata"].is_object());
        assert_eq!(parsed["metadata"]["title"], "Test Presentation");
        assert_eq!(parsed["metadata"]["author"], "Test Author");
        assert_eq!(parsed["slide_count"], 1);
        assert!(parsed["slides"].is_array());
    }

    #[test]
    fn test_json_ppt_data_without_metadata() {
        let metadata = PptMetadata {
            title: Some("Test Presentation".to_string()),
            author: Some("Test Author".to_string()),
            subject: None,
            creator: None,
            creation_date: None,
            modification_date: None,
            slide_count: 1,
            file_size: 1024,
            application: None,
        };

        let slide = Slide {
            number: 1,
            title: Some("Slide 1".to_string()),
            content: vec!["Content".to_string()],
            notes: None,
            tables: vec![],
            lists: vec![],
        };

        let ppt_data = PptData {
            slides: vec![slide],
            metadata,
            slide_count: 1,
        };

        let output = JsonOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_ppt_data(&ppt_data, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert!(parsed["metadata"].is_null());
        assert_eq!(parsed["slide_count"], 1);
        assert!(parsed["slides"].is_array());
    }
}