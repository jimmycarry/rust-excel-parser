use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::parser::{ExcelData, Sheet};
use super::OutputWriter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonWorkbook {
    pub sheets: Vec<JsonSheet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSheet {
    pub name: String,
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<Vec<String>>,
}

pub struct JsonOutput {
    pretty: bool,
}

impl JsonOutput {
    pub fn new(pretty: bool) -> Self {
        Self { pretty }
    }
}

impl OutputWriter for JsonOutput {
    fn write_excel_data<W: Write>(&self, data: &ExcelData, writer: &mut W) -> Result<()> {
        let json_workbook = JsonWorkbook {
            sheets: data.sheets.iter().map(|sheet| {
                JsonSheet {
                    name: sheet.name.clone(),
                    rows: sheet.data.len(),
                    columns: sheet.data.first().map(|row| row.len()).unwrap_or(0),
                    data: sheet.data.clone(),
                }
            }).collect(),
        };

        let json_str = if self.pretty {
            serde_json::to_string_pretty(&json_workbook)?
        } else {
            serde_json::to_string(&json_workbook)?
        };

        writer.write_all(json_str.as_bytes())?;
        writer.write_all(b"\n")?;
        Ok(())
    }

    fn write_sheet<W: Write>(&self, sheet: &Sheet, writer: &mut W) -> Result<()> {
        let json_sheet = JsonSheet {
            name: sheet.name.clone(),
            rows: sheet.data.len(),
            columns: sheet.data.first().map(|row| row.len()).unwrap_or(0),
            data: sheet.data.clone(),
        };

        let json_str = if self.pretty {
            serde_json::to_string_pretty(&json_sheet)?
        } else {
            serde_json::to_string(&json_sheet)?
        };

        writer.write_all(json_str.as_bytes())?;
        writer.write_all(b"\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Sheet;

    #[test]
    fn test_json_output() {
        let sheet = Sheet {
            name: "TestSheet".to_string(),
            data: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "25".to_string()],
            ],
        };

        let json_output = JsonOutput::new(false);
        let mut buffer = Vec::new();
        json_output.write_sheet(&sheet, &mut buffer).unwrap();
        
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("TestSheet"));
        assert!(result.contains("Name"));
        assert!(result.contains("John"));
    }

    #[test]
    fn test_json_pretty_output() {
        let sheet = Sheet {
            name: "TestSheet".to_string(),
            data: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "25".to_string()],
            ],
        };

        let json_output = JsonOutput::new(true);
        let mut buffer = Vec::new();
        json_output.write_sheet(&sheet, &mut buffer).unwrap();
        
        let result = String::from_utf8(buffer).unwrap();
        // Pretty format should have indentation
        assert!(result.contains("  "));
        assert!(result.contains("TestSheet"));
    }
}