use calamine::{open_workbook_auto, Data, Reader, Sheets};
use std::path::Path;
use crate::error::{ExcelParserError, Result};

pub struct ExcelParser;

#[derive(Debug, Clone)]
pub struct ExcelData {
    pub sheets: Vec<Sheet>,
}

#[derive(Debug, Clone)]
pub struct Sheet {
    pub name: String,
    pub data: Vec<Vec<String>>,
}

impl ExcelParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse Excel file and return structured data
    pub fn parse<P: AsRef<Path>>(&self, file_path: P) -> Result<ExcelData> {
        let file_path = file_path.as_ref();
        
        // Check if file exists
        if !file_path.exists() {
            return Err(ExcelParserError::FileNotFound(file_path.display().to_string()));
        }

        // Get file extension to determine format
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "xlsx" | "xlsm" | "xlsb" => self.parse_xlsx(file_path),
            "xls" => self.parse_xls(file_path),
            _ => Err(ExcelParserError::UnsupportedFormat(extension)),
        }
    }

    /// Parse Excel file from a specific sheet
    pub fn parse_sheet<P: AsRef<Path>>(&self, file_path: P, sheet_name: &str) -> Result<Sheet> {
        let file_path = file_path.as_ref();
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "xlsx" | "xlsm" | "xlsb" => self.parse_xlsx_sheet(file_path, sheet_name),
            "xls" => self.parse_xls_sheet(file_path, sheet_name),
            _ => Err(ExcelParserError::UnsupportedFormat(extension)),
        }
    }

    fn parse_xlsx<P: AsRef<Path>>(&self, file_path: P) -> Result<ExcelData> {
        let file_path = file_path.as_ref();
        let mut workbook: Sheets<_> = open_workbook_auto(file_path)?;
        let sheet_names = workbook.sheet_names().to_vec();
        let mut sheets = Vec::new();

        for sheet_name in sheet_names {
            if let Some(sheet_data) = self.extract_sheet_data(&mut workbook, &sheet_name)? {
                sheets.push(sheet_data);
            }
        }

        if sheets.is_empty() {
            return Err(ExcelParserError::EmptyFile);
        }

        Ok(ExcelData { sheets })
    }

    fn parse_xls<P: AsRef<Path>>(&self, file_path: P) -> Result<ExcelData> {
        // For XLS files, we use the same approach as XLSX
        // calamine handles the format differences internally
        self.parse_xlsx(file_path)
    }

    fn parse_xlsx_sheet<P: AsRef<Path>>(&self, file_path: P, sheet_name: &str) -> Result<Sheet> {
        let file_path = file_path.as_ref();
        let mut workbook: Sheets<_> = open_workbook_auto(file_path)?;
        
        self.extract_sheet_data(&mut workbook, sheet_name)?
            .ok_or_else(|| ExcelParserError::SheetNotFound(sheet_name.to_string()))
    }

    fn parse_xls_sheet<P: AsRef<Path>>(&self, file_path: P, sheet_name: &str) -> Result<Sheet> {
        // For XLS files, we use the same approach as XLSX
        self.parse_xlsx_sheet(file_path, sheet_name)
    }

    fn extract_sheet_data<R: std::io::Read + std::io::Seek>(&self, workbook: &mut Sheets<R>, sheet_name: &str) -> Result<Option<Sheet>> {
        let range = workbook.worksheet_range(sheet_name)?;
        
        if range.is_empty() {
            return Ok(None);
        }

        let mut data = Vec::new();
        
        for row in range.rows() {
            let mut row_data = Vec::new();
            for cell in row {
                let cell_value = match cell {
                    Data::Empty => String::new(),
                    Data::String(s) => s.clone(),
                    Data::Float(f) => {
                        // Check if it's a whole number
                        if f.fract() == 0.0 {
                            format!("{}", f.round() as i64)
                        } else {
                            format!("{}", f)
                        }
                    }
                    Data::Int(i) => i.to_string(),
                    Data::Bool(b) => b.to_string(),
                    Data::DateTime(dt) => dt.to_string(),
                    Data::Error(e) => format!("#ERROR: {:?}", e),
                    Data::DateTimeIso(dt) => dt.to_string(),
                    Data::DurationIso(d) => d.to_string(),
                };
                row_data.push(cell_value);
            }
            
            // Only add non-empty rows or rows with at least one non-empty cell
            if !row_data.iter().all(|cell| cell.is_empty()) {
                data.push(row_data);
            }
        }

        if data.is_empty() {
            return Ok(None);
        }

        Ok(Some(Sheet {
            name: sheet_name.to_string(),
            data,
        }))
    }

    /// Get sheet names from Excel file
    pub fn get_sheet_names<P: AsRef<Path>>(&self, file_path: P) -> Result<Vec<String>> {
        let file_path = file_path.as_ref();
        let workbook: Sheets<_> = open_workbook_auto(file_path)?;
        Ok(workbook.sheet_names().to_vec())
    }
}

impl Default for ExcelParser {
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
        
        let parser = ExcelParser::new();
        let result = parser.parse("test.txt");
        
        // Clean up
        std::fs::remove_file("test.txt").unwrap();
        
        assert!(matches!(result, Err(ExcelParserError::UnsupportedFormat(_))));
    }

    #[test]
    fn test_file_not_found() {
        let parser = ExcelParser::new();
        let result = parser.parse("nonexistent.xlsx");
        assert!(matches!(result, Err(ExcelParserError::FileNotFound(_))));
    }
}