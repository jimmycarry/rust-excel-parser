use std::io::Write;
use crate::error::Result;
use crate::parser::{ExcelData, Sheet};
use super::OutputWriter;

pub struct TableOutput {
    max_width: Option<usize>,
    borders: bool,
}

impl TableOutput {
    pub fn new(max_width: Option<usize>, borders: bool) -> Self {
        Self { max_width, borders }
    }

    fn create_table(&self, data: &[Vec<String>]) -> String {
        if data.is_empty() {
            return String::new();
        }

        // Create a simple table display without using Tabled trait
        let mut output = String::new();
        
        if self.borders {
            self.create_bordered_table(data, &mut output);
        } else {
            self.create_simple_table(data, &mut output);
        }

        output
    }

    fn create_bordered_table(&self, data: &[Vec<String>], output: &mut String) {
        if data.is_empty() {
            return;
        }

        let max_cols = data.iter().map(|row| row.len()).max().unwrap_or(0);
        let mut col_widths = vec![0; max_cols];

        // Calculate column widths
        for row in data {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }

        // Apply max width constraint
        if let Some(max_width) = self.max_width {
            let total_width: usize = col_widths.iter().sum();
            if total_width > max_width {
                let avg_width = max_width / max_cols;
                for width in &mut col_widths {
                    *width = (*width).min(avg_width);
                }
            }
        }

        // Create top border
        output.push('┌');
        for (i, &width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(width + 2));
            if i < col_widths.len() - 1 {
                output.push('┬');
            }
        }
        output.push_str("┐\n");

        // Create rows
        for (row_idx, row) in data.iter().enumerate() {
            output.push('│');
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx < col_widths.len() {
                    let width = col_widths[col_idx];
                    let truncated = if cell.len() > width {
                        format!("{}…", &cell[..width.saturating_sub(1)])
                    } else {
                        cell.clone()
                    };
                    output.push_str(&format!(" {:<width$} ", truncated, width = width));
                    if col_idx < col_widths.len() - 1 {
                        output.push('│');
                    }
                }
            }
            output.push_str("│\n");

            // Add separator after header row
            if row_idx == 0 && data.len() > 1 {
                output.push('├');
                for (i, &width) in col_widths.iter().enumerate() {
                    output.push_str(&"─".repeat(width + 2));
                    if i < col_widths.len() - 1 {
                        output.push('┼');
                    }
                }
                output.push_str("┤\n");
            }
        }

        // Create bottom border
        output.push('└');
        for (i, &width) in col_widths.iter().enumerate() {
            output.push_str(&"─".repeat(width + 2));
            if i < col_widths.len() - 1 {
                output.push('┴');
            }
        }
        output.push_str("┘\n");
    }

    fn create_simple_table(&self, data: &[Vec<String>], output: &mut String) {
        let max_cols = data.iter().map(|row| row.len()).max().unwrap_or(0);
        let mut col_widths = vec![0; max_cols];

        // Calculate column widths
        for row in data {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }

        // Apply max width constraint
        if let Some(max_width) = self.max_width {
            let total_width: usize = col_widths.iter().sum();
            if total_width > max_width {
                let avg_width = max_width / max_cols;
                for width in &mut col_widths {
                    *width = (*width).min(avg_width);
                }
            }
        }

        // Create rows
        for row in data {
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx < col_widths.len() {
                    let width = col_widths[col_idx];
                    let truncated = if cell.len() > width {
                        format!("{}…", &cell[..width.saturating_sub(1)])
                    } else {
                        cell.clone()
                    };
                    output.push_str(&format!("{:<width$}", truncated, width = width));
                    if col_idx < col_widths.len() - 1 {
                        output.push_str("  ");
                    }
                }
            }
            output.push('\n');
        }
    }
}

impl OutputWriter for TableOutput {
    fn write_excel_data<W: Write>(&self, data: &ExcelData, writer: &mut W) -> Result<()> {
        for (i, sheet) in data.sheets.iter().enumerate() {
            if i > 0 {
                writer.write_all(b"\n\n")?;
            }
            
            // Write sheet header
            writer.write_all(format!("=== Sheet: {} ===\n", sheet.name).as_bytes())?;
            
            if !sheet.data.is_empty() {
                let table_str = self.create_table(&sheet.data);
                writer.write_all(table_str.as_bytes())?;
                writer.write_all(b"\n")?;
            } else {
                writer.write_all(b"(Empty sheet)\n")?;
            }
        }
        Ok(())
    }

    fn write_sheet<W: Write>(&self, sheet: &Sheet, writer: &mut W) -> Result<()> {
        // Write sheet header
        writer.write_all(format!("=== Sheet: {} ===\n", sheet.name).as_bytes())?;
        
        if !sheet.data.is_empty() {
            let table_str = self.create_table(&sheet.data);
            writer.write_all(table_str.as_bytes())?;
            writer.write_all(b"\n")?;
        } else {
            writer.write_all(b"(Empty sheet)\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Sheet;

    #[test]
    fn test_table_output() {
        let sheet = Sheet {
            name: "TestSheet".to_string(),
            data: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "25".to_string()],
                vec!["Jane".to_string(), "30".to_string()],
            ],
        };

        let table_output = TableOutput::new(None, true);
        let mut buffer = Vec::new();
        table_output.write_sheet(&sheet, &mut buffer).unwrap();
        
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("TestSheet"));
        assert!(result.contains("Name"));
        assert!(result.contains("John"));
        assert!(result.contains("Jane"));
    }

    #[test]
    fn test_table_no_borders() {
        let sheet = Sheet {
            name: "TestSheet".to_string(),
            data: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "25".to_string()],
            ],
        };

        let table_output = TableOutput::new(None, false);
        let mut buffer = Vec::new();
        table_output.write_sheet(&sheet, &mut buffer).unwrap();
        
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("TestSheet"));
        assert!(result.contains("Name"));
        assert!(result.contains("John"));
    }

    #[test]
    fn test_empty_sheet() {
        let sheet = Sheet {
            name: "EmptySheet".to_string(),
            data: vec![],
        };

        let table_output = TableOutput::new(None, true);
        let mut buffer = Vec::new();
        table_output.write_sheet(&sheet, &mut buffer).unwrap();
        
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("EmptySheet"));
        assert!(result.contains("(Empty sheet)"));
    }
}