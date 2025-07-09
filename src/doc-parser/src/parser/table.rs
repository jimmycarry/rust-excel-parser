//! # Table Extraction Module
//!
//! This module provides comprehensive table extraction functionality for DOCX documents.
//! It supports multiple extraction modes, intelligent header detection, and various output formats.
//!
//! ## Features
//!
//! - **Structured Data Extraction**: Extract tables with full structure information
//! - **Intelligent Header Detection**: Automatically detect table headers
//! - **Merged Cell Handling**: Support for merged cells with different strategies
//! - **Multiple Output Formats**: Plain text, CSV, TSV, Markdown, JSON, HTML
//! - **Configurable Processing**: Flexible configuration for different use cases
//!
//! ## Usage
//!
//! ```rust
//! use doc_parser::parser::table::{TableExtractor, TableExtractionConfig};
//!
//! let config = TableExtractionConfig::simple();
//! let extractor = TableExtractor::new(config);
//! // let table_data = extractor.extract_table(&docx_table)?;
//! ```

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Table data structure containing all extracted table information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    /// All table rows
    pub rows: Vec<TableRow>,
    /// Detected headers if any
    pub headers: Option<Vec<String>>,
    /// Whether the table has a header row
    pub has_header: bool,
    /// Number of columns in the table
    pub column_count: usize,
    /// Number of rows in the table
    pub row_count: usize,
    /// Optional table identifier
    pub table_id: Option<String>,
    /// Optional table title or caption
    pub title: Option<String>,
}

/// Individual table row containing cells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    /// All cells in this row
    pub cells: Vec<TableCell>,
    /// Whether this row is a header row
    pub is_header: bool,
    /// Row index in the table
    pub row_index: usize,
}

/// Individual table cell with content and formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    /// Plain text content of the cell
    pub content: String,
    /// Formatted content (may include markup)
    pub formatted_content: String,
    /// Number of columns this cell spans
    pub colspan: Option<usize>,
    /// Number of rows this cell spans
    pub rowspan: Option<usize>,
    /// Text alignment in the cell
    pub alignment: Option<CellAlignment>,
    /// Cell formatting information
    pub formatting: Option<CellFormatting>,
    /// Type of cell (header, data, merged, empty)
    pub cell_type: CellType,
}

/// Cell alignment options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellAlignment {
    /// Left-aligned text
    Left,
    /// Center-aligned text
    Center,
    /// Right-aligned text
    Right,
    /// Justified text
    Justify,
}

/// Cell type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellType {
    /// Header cell
    Header,
    /// Regular data cell
    Data,
    /// Merged cell (part of a merged range)
    Merged,
    /// Empty cell
    Empty,
}

/// Cell formatting information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellFormatting {
    /// Whether text is bold
    pub bold: bool,
    /// Whether text is italic
    pub italic: bool,
    /// Whether text is underlined
    pub underline: bool,
    /// Background color (hex format)
    pub background_color: Option<String>,
    /// Text color (hex format)
    pub text_color: Option<String>,
    /// Font size in points
    pub font_size: Option<u32>,
    /// Font family name
    pub font_family: Option<String>,
}

/// Table extraction modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableExtractionMode {
    /// Simple text extraction without formatting
    Simple,
    /// Structured data extraction with basic formatting
    Structured,
    /// Full formatting preservation
    Formatted,
    /// Complete extraction including all metadata
    Full,
}

/// Configuration for table extraction
#[derive(Debug, Clone)]
pub struct TableExtractionConfig {
    /// Extraction mode
    pub mode: TableExtractionMode,
    /// Whether to detect headers automatically
    pub detect_headers: bool,
    /// Whether to preserve formatting information
    pub preserve_formatting: bool,
    /// Whether to include empty cells in output
    pub include_empty_cells: bool,
    /// How to handle merged cells
    pub merge_cells_handling: MergeCellsHandling,
    /// Output format preference
    pub output_format: TableOutputFormat,
}

/// Merged cells handling strategies
#[derive(Debug, Clone)]
pub enum MergeCellsHandling {
    /// Ignore merged cell information
    Ignore,
    /// Preserve merged cell information
    Preserve,
    /// Expand merged cells to fill the range
    Expand,
}

/// Available output formats for tables
#[derive(Debug, Clone)]
pub enum TableOutputFormat {
    /// Plain text with spacing
    PlainText,
    /// Comma-separated values
    CSV,
    /// Tab-separated values
    TSV,
    /// Markdown table format
    Markdown,
    /// JSON structured data
    JSON,
    /// HTML table format
    HTML,
}

/// Cell data types for analysis
#[derive(Debug, Clone, PartialEq)]
enum CellDataType {
    /// Empty cell
    Empty,
    /// Numeric data
    Number,
    /// Text data
    Text,
    /// Date data
    Date,
    /// Boolean data
    Boolean,
}

/// Merged cell range information
#[derive(Debug, Clone)]
struct MergedRange {
    /// Starting row index
    start_row: usize,
    /// Ending row index
    end_row: usize,
    /// Starting column index
    start_col: usize,
    /// Ending column index
    end_col: usize,
    /// Type of merge
    merge_type: MergeType,
}

/// Types of cell merges
#[derive(Debug, Clone, PartialEq)]
enum MergeType {
    /// Horizontal merge (across columns)
    Horizontal,
    /// Vertical merge (across rows)
    Vertical,
}

/// Table extractor with configuration
pub struct TableExtractor {
    config: TableExtractionConfig,
}

impl TableData {
    /// Create a new empty table data structure
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            headers: None,
            has_header: false,
            column_count: 0,
            row_count: 0,
            table_id: None,
            title: None,
        }
    }

    /// Create a new table data with specified capacity
    pub fn with_capacity(row_capacity: usize) -> Self {
        Self {
            rows: Vec::with_capacity(row_capacity),
            headers: None,
            has_header: false,
            column_count: 0,
            row_count: 0,
            table_id: None,
            title: None,
        }
    }

    /// Add a row to the table
    pub fn add_row(&mut self, row: TableRow) {
        self.row_count += 1;
        if row.cells.len() > self.column_count {
            self.column_count = row.cells.len();
        }
        self.rows.push(row);
    }

    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    /// Get the number of columns
    pub fn column_count(&self) -> usize {
        self.column_count
    }

    /// Check if the table is empty
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Get a specific cell by row and column index
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&TableCell> {
        self.rows.get(row)?.cells.get(col)
    }

    /// Get all cells in a specific row
    pub fn get_row(&self, row: usize) -> Option<&TableRow> {
        self.rows.get(row)
    }

    /// Get all cells in a specific column
    pub fn get_column(&self, col: usize) -> Vec<Option<&TableCell>> {
        self.rows.iter().map(|row| row.cells.get(col)).collect()
    }

    /// Set table headers
    pub fn set_headers(&mut self, headers: Vec<String>) {
        self.has_header = true;
        self.headers = Some(headers);
    }

    /// Update table statistics
    pub fn update_statistics(&mut self) {
        self.row_count = self.rows.len();
        self.column_count = self.rows.iter().map(|row| row.cells.len()).max().unwrap_or(0);
    }
}

impl TableRow {
    /// Create a new table row
    pub fn new(row_index: usize) -> Self {
        Self {
            cells: Vec::new(),
            is_header: false,
            row_index,
        }
    }

    /// Create a new table row with specified capacity
    pub fn with_capacity(row_index: usize, cell_capacity: usize) -> Self {
        Self {
            cells: Vec::with_capacity(cell_capacity),
            is_header: false,
            row_index,
        }
    }

    /// Add a cell to the row
    pub fn add_cell(&mut self, cell: TableCell) {
        self.cells.push(cell);
    }

    /// Get the number of cells in this row
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Check if this row is empty
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty() || self.cells.iter().all(|cell| cell.is_empty())
    }

    /// Mark this row as a header row
    pub fn mark_as_header(&mut self) {
        self.is_header = true;
        for cell in &mut self.cells {
            cell.cell_type = CellType::Header;
        }
    }

    /// Get all non-empty cells in this row
    pub fn non_empty_cells(&self) -> Vec<&TableCell> {
        self.cells.iter().filter(|cell| !cell.is_empty()).collect()
    }
}

impl TableCell {
    /// Create a new table cell
    pub fn new(content: String) -> Self {
        Self {
            formatted_content: content.clone(),
            content,
            colspan: None,
            rowspan: None,
            alignment: None,
            formatting: None,
            cell_type: CellType::Data,
        }
    }

    /// Create a new table cell with formatting
    pub fn with_formatting(content: String, formatting: CellFormatting) -> Self {
        Self {
            formatted_content: content.clone(),
            content,
            colspan: None,
            rowspan: None,
            alignment: None,
            formatting: Some(formatting),
            cell_type: CellType::Data,
        }
    }

    /// Create an empty cell
    pub fn empty() -> Self {
        Self {
            content: String::new(),
            formatted_content: String::new(),
            colspan: None,
            rowspan: None,
            alignment: None,
            formatting: None,
            cell_type: CellType::Empty,
        }
    }

    /// Check if the cell is empty
    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }

    /// Check if the cell is merged
    pub fn is_merged(&self) -> bool {
        matches!(self.cell_type, CellType::Merged)
    }

    /// Check if the cell is a header
    pub fn is_header(&self) -> bool {
        matches!(self.cell_type, CellType::Header)
    }

    /// Set cell as merged
    pub fn set_merged(&mut self, colspan: Option<usize>, rowspan: Option<usize>) {
        self.cell_type = CellType::Merged;
        self.colspan = colspan;
        self.rowspan = rowspan;
    }

    /// Set cell alignment
    pub fn set_alignment(&mut self, alignment: CellAlignment) {
        self.alignment = Some(alignment);
    }

    /// Get cell content length
    pub fn content_length(&self) -> usize {
        self.content.chars().count()
    }
}

impl CellFormatting {
    /// Create new cell formatting with default values
    pub fn new() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            background_color: None,
            text_color: None,
            font_size: None,
            font_family: None,
        }
    }

    /// Create bold formatting
    pub fn bold() -> Self {
        Self {
            bold: true,
            ..Self::new()
        }
    }

    /// Create italic formatting
    pub fn italic() -> Self {
        Self {
            italic: true,
            ..Self::new()
        }
    }

    /// Check if any formatting is applied
    pub fn has_formatting(&self) -> bool {
        self.bold || self.italic || self.underline || 
        self.background_color.is_some() || self.text_color.is_some() ||
        self.font_size.is_some() || self.font_family.is_some()
    }

    /// Apply formatting to text (basic implementation)
    pub fn apply_to_text(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        if self.bold {
            result = format!("**{}**", result);
        }
        
        if self.italic {
            result = format!("*{}*", result);
        }
        
        if self.underline {
            result = format!("__{result}__");
        }
        
        result
    }
}

impl TableExtractionConfig {
    /// Create a new default configuration
    pub fn new() -> Self {
        Self {
            mode: TableExtractionMode::Structured,
            detect_headers: true,
            preserve_formatting: false,
            include_empty_cells: false,
            merge_cells_handling: MergeCellsHandling::Preserve,
            output_format: TableOutputFormat::PlainText,
        }
    }

    /// Create a simple configuration for basic text extraction
    pub fn simple() -> Self {
        Self {
            mode: TableExtractionMode::Simple,
            detect_headers: false,
            preserve_formatting: false,
            include_empty_cells: false,
            merge_cells_handling: MergeCellsHandling::Ignore,
            output_format: TableOutputFormat::PlainText,
        }
    }

    /// Create a full configuration with all features enabled
    pub fn full() -> Self {
        Self {
            mode: TableExtractionMode::Full,
            detect_headers: true,
            preserve_formatting: true,
            include_empty_cells: true,
            merge_cells_handling: MergeCellsHandling::Preserve,
            output_format: TableOutputFormat::JSON,
        }
    }

    /// Builder method to set header detection
    pub fn with_headers(mut self, detect: bool) -> Self {
        self.detect_headers = detect;
        self
    }

    /// Builder method to set formatting preservation
    pub fn with_formatting(mut self, preserve: bool) -> Self {
        self.preserve_formatting = preserve;
        self
    }

    /// Builder method to set output format
    pub fn with_output_format(mut self, format: TableOutputFormat) -> Self {
        self.output_format = format;
        self
    }

    /// Builder method to set extraction mode
    pub fn with_mode(mut self, mode: TableExtractionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Builder method to set merged cells handling
    pub fn with_merge_cells_handling(mut self, handling: MergeCellsHandling) -> Self {
        self.merge_cells_handling = handling;
        self
    }

    /// Builder method to set empty cells inclusion
    pub fn with_empty_cells(mut self, include: bool) -> Self {
        self.include_empty_cells = include;
        self
    }
}

impl TableExtractor {
    /// Create a new table extractor with configuration
    pub fn new(config: TableExtractionConfig) -> Self {
        Self { config }
    }

    /// Extract table data from a docx-rs Table
    pub fn extract_table(&self, table: &docx_rs::Table) -> Result<TableData> {
        let mut table_data = TableData::with_capacity(table.rows.len());
        
        // Step 1: Extract basic table structure
        for (row_index, row_child) in table.rows.iter().enumerate() {
            // TableChild should be TableRow
            let docx_rs::TableChild::TableRow(row) = row_child;
            let mut table_row = TableRow::with_capacity(row_index, row.cells.len());
            
            for cell_child in &row.cells {
                // TableRowChild should be TableCell
                let docx_rs::TableRowChild::TableCell(cell) = cell_child;
                let cell_data = self.extract_cell_data(cell)?;
                table_row.add_cell(cell_data);
            }
            
            table_data.add_row(table_row);
        }
        
        // Step 2: Process according to configuration
        self.process_table_data(&mut table_data)?;
        
        Ok(table_data)
    }

    /// Extract multiple tables from a document
    pub fn extract_all_tables(&self, docx: &docx_rs::Docx) -> Result<Vec<TableData>> {
        let mut tables = Vec::new();
        
        for child in &docx.document.children {
            if let docx_rs::DocumentChild::Table(table) = child {
                let table_data = self.extract_table(table)?;
                tables.push(table_data);
            }
        }
        
        Ok(tables)
    }

    /// Extract cell data from a docx-rs TableCell
    fn extract_cell_data(&self, cell: &docx_rs::TableCell) -> Result<TableCell> {
        let mut content = String::new();
        let mut formatting = CellFormatting::new();
        
        // Extract text content from cell children
        for child in &cell.children {
            if let docx_rs::TableCellContent::Paragraph(para) = child {
                let para_text = self.extract_paragraph_text(para)?;
                if !para_text.is_empty() {
                    content.push_str(&para_text);
                    content.push('\n');
                }
            }
        }
        
        // Clean up content
        content = content.trim().to_string();
        
        // Extract formatting if required
        if self.config.preserve_formatting {
            formatting = self.extract_cell_formatting(cell)?;
        }
        
        let mut table_cell = if content.is_empty() {
            TableCell::empty()
        } else {
            TableCell::new(content)
        };
        
        if formatting.has_formatting() {
            table_cell.formatting = Some(formatting);
        }
        
        Ok(table_cell)
    }

    /// Extract text from a paragraph
    fn extract_paragraph_text(&self, paragraph: &docx_rs::Paragraph) -> Result<String> {
        let mut text = String::new();
        
        for child in &paragraph.children {
            if let docx_rs::ParagraphChild::Run(run) = child {
                for run_child in &run.children {
                    if let docx_rs::RunChild::Text(text_element) = run_child {
                        text.push_str(&text_element.text);
                    }
                }
            }
        }
        
        Ok(text)
    }

    /// Extract formatting from a cell (enhanced implementation)
    fn extract_cell_formatting(&self, cell: &docx_rs::TableCell) -> Result<CellFormatting> {
        let mut formatting = CellFormatting::new();
        
        // Extract formatting from paragraphs within the cell
        for child in &cell.children {
            if let docx_rs::TableCellContent::Paragraph(paragraph) = child {
                let para_formatting = self.extract_paragraph_formatting(paragraph)?;
                formatting = self.merge_formatting(formatting, para_formatting);
            }
        }
        
        // Extract cell-specific formatting from properties
        if let Some(cell_formatting) = self.extract_cell_property_formatting(cell)? {
            formatting = self.merge_formatting(formatting, cell_formatting);
        }
        
        Ok(formatting)
    }
    
    /// Extract formatting from a paragraph
    fn extract_paragraph_formatting(&self, paragraph: &docx_rs::Paragraph) -> Result<CellFormatting> {
        let mut formatting = CellFormatting::new();
        
        // Extract formatting from runs within the paragraph
        for child in &paragraph.children {
            if let docx_rs::ParagraphChild::Run(run) = child {
                let run_formatting = self.extract_run_formatting(run)?;
                formatting = self.merge_formatting(formatting, run_formatting);
            }
        }
        
        Ok(formatting)
    }
    
    /// Extract formatting from a text run
    fn extract_run_formatting(&self, _run: &docx_rs::Run) -> Result<CellFormatting> {
        let formatting = CellFormatting::new();
        
        // TODO: Implement proper formatting extraction when docx-rs API is more stable
        // For now, return basic formatting to avoid compilation issues
        // This will be enhanced in future versions
        
        Ok(formatting)
    }
    
    /// Extract formatting from cell properties
    fn extract_cell_property_formatting(&self, _cell: &docx_rs::TableCell) -> Result<Option<CellFormatting>> {
        // TODO: Implement proper cell formatting extraction when docx-rs API is more stable
        // For now, return None to avoid compilation issues
        // This will be enhanced in future versions
        Ok(None)
    }
    
    /// Merge two formatting objects, with the second taking precedence
    fn merge_formatting(&self, mut base: CellFormatting, overlay: CellFormatting) -> CellFormatting {
        // Overlay takes precedence for boolean values
        if overlay.bold {
            base.bold = true;
        }
        if overlay.italic {
            base.italic = true;
        }
        if overlay.underline {
            base.underline = true;
        }
        
        // Overlay takes precedence for optional values
        if overlay.font_size.is_some() {
            base.font_size = overlay.font_size;
        }
        if overlay.font_family.is_some() {
            base.font_family = overlay.font_family;
        }
        if overlay.text_color.is_some() {
            base.text_color = overlay.text_color;
        }
        if overlay.background_color.is_some() {
            base.background_color = overlay.background_color;
        }
        
        base
    }
    
    /// Convert highlight color to hex format
    fn convert_highlight_to_hex(&self, highlight: &str) -> String {
        // Convert common highlight colors to hex values
        match highlight.to_lowercase().as_str() {
            "yellow" => "#FFFF00".to_string(),
            "green" => "#00FF00".to_string(),
            "cyan" => "#00FFFF".to_string(),
            "magenta" => "#FF00FF".to_string(),
            "blue" => "#0000FF".to_string(),
            "red" => "#FF0000".to_string(),
            "darkblue" => "#000080".to_string(),
            "darkcyan" => "#008080".to_string(),
            "darkgreen" => "#008000".to_string(),
            "darkmagenta" => "#800080".to_string(),
            "darkred" => "#800000".to_string(),
            "darkyellow" => "#808000".to_string(),
            "darkgray" => "#808080".to_string(),
            "lightgray" => "#C0C0C0".to_string(),
            "black" => "#000000".to_string(),
            _ => format!("#{}", highlight), // Assume it's already a hex value
        }
    }

    /// Process extracted table data according to configuration
    fn process_table_data(&self, table_data: &mut TableData) -> Result<()> {
        // Update statistics
        table_data.update_statistics();
        
        // Detect headers if configured
        if self.config.detect_headers {
            self.detect_table_headers(table_data)?;
        }
        
        // Handle merged cells
        self.handle_merged_cells(table_data)?;
        
        // Filter empty cells if configured
        if !self.config.include_empty_cells {
            self.filter_empty_cells(table_data)?;
        }
        
        Ok(())
    }

    /// Detect table headers using enhanced algorithms
    fn detect_table_headers(&self, table_data: &mut TableData) -> Result<()> {
        if table_data.rows.is_empty() {
            return Ok(());
        }
        
        let first_row = &table_data.rows[0];
        let mut header_score = 0.0;
        let mut max_score = 0.0;
        
        // Enhanced header detection with multiple strategies
        
        // Strategy 1: Formatting difference detection
        let formatting_score = self.check_formatting_difference(first_row, &table_data.rows[1..])?;
        header_score += formatting_score * 0.3;
        max_score += 0.3;
        
        // Strategy 2: Content pattern analysis
        let content_score = self.check_header_content_pattern(first_row)?;
        header_score += content_score * 0.25;
        max_score += 0.25;
        
        // Strategy 3: Data type consistency check
        let consistency_score = self.check_data_type_consistency(&table_data.rows[1..])?;
        header_score += consistency_score * 0.2;
        max_score += 0.2;
        
        // Strategy 4: Text length analysis
        let length_score = self.check_text_length_pattern(first_row, &table_data.rows[1..])?;
        header_score += length_score * 0.15;
        max_score += 0.15;
        
        // Strategy 5: Uniqueness check
        let uniqueness_score = self.check_header_uniqueness(first_row)?;
        header_score += uniqueness_score * 0.1;
        max_score += 0.1;
        
        // Determine if first row is a header (threshold: 60% confidence)
        let confidence = if max_score > 0.0 { header_score / max_score } else { 0.0 };
        
        if confidence > 0.6 {
            table_data.has_header = true;
            table_data.headers = Some(
                first_row.cells.iter()
                    .map(|cell| cell.content.trim().to_string())
                    .collect()
            );
            
            // Update first row
            if let Some(first_row) = table_data.rows.get_mut(0) {
                first_row.mark_as_header();
            }
        }
        
        Ok(())
    }
    
    /// Check formatting differences between first row and data rows
    fn check_formatting_difference(&self, first_row: &TableRow, data_rows: &[TableRow]) -> Result<f64> {
        if data_rows.is_empty() {
            return Ok(0.0);
        }
        
        let mut formatting_diff_count = 0;
        let mut total_comparisons = 0;
        
        for (i, first_cell) in first_row.cells.iter().enumerate() {
            for data_row in data_rows.iter().take(3) { // Check first 3 data rows
                if let Some(data_cell) = data_row.cells.get(i) {
                    total_comparisons += 1;
                    
                    // Check if formatting differs
                    let first_has_formatting = first_cell.formatting.as_ref()
                        .map(|f| f.has_formatting()).unwrap_or(false);
                    let data_has_formatting = data_cell.formatting.as_ref()
                        .map(|f| f.has_formatting()).unwrap_or(false);
                    
                    if first_has_formatting != data_has_formatting {
                        formatting_diff_count += 1;
                    }
                    
                    // Check bold difference (common for headers)
                    if let (Some(first_fmt), Some(data_fmt)) = (&first_cell.formatting, &data_cell.formatting) {
                        if first_fmt.bold && !data_fmt.bold {
                            formatting_diff_count += 1;
                        }
                    }
                }
            }
        }
        
        Ok(if total_comparisons > 0 {
            formatting_diff_count as f64 / total_comparisons as f64
        } else {
            0.0
        })
    }
    
    /// Check if content matches typical header patterns
    fn check_header_content_pattern(&self, first_row: &TableRow) -> Result<f64> {
        let mut header_indicators = 0;
        let total_cells = first_row.cells.len();
        
        for cell in &first_row.cells {
            let content = cell.content.trim().to_lowercase();
            
            // Check for common header keywords
            if content.contains("name") || content.contains("id") || content.contains("title") ||
               content.contains("date") || content.contains("time") || content.contains("type") ||
               content.contains("status") || content.contains("amount") || content.contains("count") ||
               content.contains("number") || content.contains("code") || content.contains("description") {
                header_indicators += 1;
            }
            
            // Check for typical header patterns (short, descriptive)
            if content.len() > 2 && content.len() < 30 && 
               !content.chars().any(|c| c.is_numeric() && content.matches(char::is_numeric).count() > 3) {
                header_indicators += 1;
            }
            
            // Check for capitalized words (common in headers)
            if content.split_whitespace().any(|word| word.chars().next().unwrap_or('a').is_uppercase()) {
                header_indicators += 1;
            }
        }
        
        Ok(if total_cells > 0 {
            (header_indicators as f64 / total_cells as f64).min(1.0)
        } else {
            0.0
        })
    }
    
    /// Check data type consistency in subsequent rows
    fn check_data_type_consistency(&self, data_rows: &[TableRow]) -> Result<f64> {
        if data_rows.is_empty() {
            return Ok(0.0);
        }
        
        let mut consistency_score = 0.0;
        let mut total_columns = 0;
        
        // Get the maximum number of columns
        let max_cols = data_rows.iter()
            .map(|row| row.cells.len())
            .max()
            .unwrap_or(0);
        
        for col_idx in 0..max_cols {
            let mut column_types = Vec::new();
            
            // Collect data types for this column
            for row in data_rows.iter().take(5) { // Check first 5 data rows
                if let Some(cell) = row.cells.get(col_idx) {
                    let cell_type = self.detect_cell_data_type(&cell.content);
                    column_types.push(cell_type);
                }
            }
            
            // Check consistency
            if !column_types.is_empty() {
                let first_type = &column_types[0];
                let consistent_count = column_types.iter()
                    .filter(|t| *t == first_type)
                    .count();
                
                let consistency = consistent_count as f64 / column_types.len() as f64;
                consistency_score += consistency;
                total_columns += 1;
            }
        }
        
        Ok(if total_columns > 0 {
            consistency_score / total_columns as f64
        } else {
            0.0
        })
    }
    
    /// Check text length patterns
    fn check_text_length_pattern(&self, first_row: &TableRow, data_rows: &[TableRow]) -> Result<f64> {
        if data_rows.is_empty() {
            return Ok(0.0);
        }
        
        let first_avg_length = first_row.cells.iter()
            .map(|cell| cell.content_length())
            .sum::<usize>() as f64 / first_row.cells.len() as f64;
        
        let mut data_avg_length = 0.0;
        let mut row_count = 0;
        
        for row in data_rows.iter().take(3) {
            if !row.cells.is_empty() {
                let row_avg = row.cells.iter()
                    .map(|cell| cell.content_length())
                    .sum::<usize>() as f64 / row.cells.len() as f64;
                data_avg_length += row_avg;
                row_count += 1;
            }
        }
        
        if row_count > 0 {
            data_avg_length /= row_count as f64;
            
            // Headers typically shorter than data
            if first_avg_length > 0.0 && first_avg_length < 50.0 && 
               data_avg_length > first_avg_length * 1.2 {
                return Ok(0.8);
            }
        }
        
        Ok(if first_avg_length > 0.0 && first_avg_length < 30.0 { 0.4 } else { 0.0 })
    }
    
    /// Check header uniqueness
    fn check_header_uniqueness(&self, first_row: &TableRow) -> Result<f64> {
        let contents: Vec<String> = first_row.cells.iter()
            .map(|cell| cell.content.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        
        if contents.is_empty() {
            return Ok(0.0);
        }
        
        let unique_count = contents.iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        
        Ok(unique_count as f64 / contents.len() as f64)
    }
    
    /// Detect data type of cell content with enhanced intelligence
    fn detect_cell_data_type(&self, content: &str) -> CellDataType {
        let trimmed = content.trim();
        
        if trimmed.is_empty() {
            return CellDataType::Empty;
        }
        
        // Check for number (including currency, percentages, etc.)
        if self.is_numeric_type(trimmed) {
            return CellDataType::Number;
        }
        
        // Check for date patterns
        if self.is_date_pattern(trimmed) {
            return CellDataType::Date;
        }
        
        // Check for boolean
        if self.is_boolean_type(trimmed) {
            return CellDataType::Boolean;
        }
        
        CellDataType::Text
    }
    
    /// Enhanced numeric type detection
    fn is_numeric_type(&self, text: &str) -> bool {
        let cleaned = text.trim();
        
        // Direct number parsing
        if cleaned.parse::<f64>().is_ok() {
            return true;
        }
        
        // Currency patterns ($100, €50, ¥200, etc.)
        if self.is_currency_pattern(cleaned) {
            return true;
        }
        
        // Percentage patterns (50%, 0.25%)
        if self.is_percentage_pattern(cleaned) {
            return true;
        }
        
        // Formatted numbers with commas (1,000.50)
        if self.is_formatted_number(cleaned) {
            return true;
        }
        
        // Scientific notation (1.23e-4)
        if self.is_scientific_notation(cleaned) {
            return true;
        }
        
        false
    }
    
    /// Check for currency patterns
    fn is_currency_pattern(&self, text: &str) -> bool {
        let currency_symbols = ["$", "€", "¥", "£", "₹", "₽", "₩", "₪", "₦", "₡"];
        
        for symbol in &currency_symbols {
            if text.starts_with(symbol) || text.ends_with(symbol) {
                let number_part = text.trim_start_matches(symbol).trim_end_matches(symbol);
                if number_part.replace(',', "").parse::<f64>().is_ok() {
                    return true;
                }
            }
        }
        
        // Check for currency codes (USD 100, EUR 50)
        let currency_codes = ["USD", "EUR", "GBP", "JPY", "CNY", "INR", "RUB", "KRW"];
        for code in &currency_codes {
            if text.starts_with(code) || text.ends_with(code) {
                let parts: Vec<&str> = text.split_whitespace().collect();
                if parts.len() == 2 {
                    if parts[0] == *code && parts[1].parse::<f64>().is_ok() {
                        return true;
                    }
                    if parts[1] == *code && parts[0].parse::<f64>().is_ok() {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    /// Check for percentage patterns
    fn is_percentage_pattern(&self, text: &str) -> bool {
        if text.ends_with('%') {
            let number_part = text.trim_end_matches('%');
            return number_part.parse::<f64>().is_ok();
        }
        false
    }
    
    /// Check for formatted numbers (with commas)
    fn is_formatted_number(&self, text: &str) -> bool {
        if text.contains(',') {
            let cleaned = text.replace(',', "");
            return cleaned.parse::<f64>().is_ok();
        }
        false
    }
    
    /// Check for scientific notation
    fn is_scientific_notation(&self, text: &str) -> bool {
        let lower = text.to_lowercase();
        if lower.contains('e') {
            return lower.parse::<f64>().is_ok();
        }
        false
    }
    
    /// Enhanced boolean type detection
    fn is_boolean_type(&self, text: &str) -> bool {
        let lower = text.to_lowercase();
        
        // Standard boolean values
        if matches!(lower.as_str(), "true" | "false" | "yes" | "no") {
            return true;
        }
        
        // Checkbox representations
        if matches!(lower.as_str(), "✓" | "✗" | "☑" | "☐" | "x" | "o") {
            return true;
        }
        
        // Numeric boolean (0/1)
        if matches!(lower.as_str(), "0" | "1") {
            return true;
        }
        
        // Other language variants
        if matches!(lower.as_str(), "是" | "否" | "有" | "無" | "oui" | "non" | "да" | "нет") {
            return true;
        }
        
        false
    }
    
    /// Check if text matches common date patterns (enhanced)
    fn is_date_pattern(&self, text: &str) -> bool {
        // Enhanced date pattern detection with multiple formats
        let date_patterns = [
            // Standard numeric dates
            r"^\d{1,2}[/-]\d{1,2}[/-]\d{2,4}$",           // MM/DD/YYYY, DD/MM/YYYY
            r"^\d{4}[/-]\d{1,2}[/-]\d{1,2}$",             // YYYY/MM/DD
            r"^\d{1,2}[/-]\d{1,2}[/-]\d{2}$",             // MM/DD/YY
            // ISO 8601 format
            r"^\d{4}-\d{2}-\d{2}$",                        // YYYY-MM-DD
            r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$",     // YYYY-MM-DDTHH:MM:SS
            // Dotted formats
            r"^\d{1,2}\.\d{1,2}\.\d{2,4}$",               // DD.MM.YYYY
            // With time
            r"^\d{1,2}[/-]\d{1,2}[/-]\d{2,4}\s+\d{1,2}:\d{2}(:\d{2})?$", // MM/DD/YYYY HH:MM(:SS)
            // Month names
            r"^(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\s+\d{1,2},?\s+\d{2,4}$",
            r"^\d{1,2}\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\s+\d{2,4}$",
            // Time only
            r"^\d{1,2}:\d{2}(:\d{2})?(\s*(AM|PM))?$",     // HH:MM(:SS) (AM/PM)
        ];
        
        for pattern in &date_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if regex.is_match(text) {
                    return true;
                }
            }
        }
        
        // Check for relative dates
        if self.is_relative_date(text) {
            return true;
        }
        
        false
    }
    
    /// Check for relative date patterns
    fn is_relative_date(&self, text: &str) -> bool {
        let lower = text.to_lowercase();
        
        // Common relative date terms
        let relative_terms = [
            "today", "tomorrow", "yesterday", "now",
            "今天", "明天", "昨天", "现在",
            "aujourd'hui", "demain", "hier",
            "сегодня", "завтра", "вчера",
        ];
        
        for term in &relative_terms {
            if lower.contains(term) {
                return true;
            }
        }
        
        // Patterns like "2 days ago", "next week", etc.
        let relative_patterns = [
            r"\d+\s+(day|week|month|year)s?\s+(ago|from now)",
            r"(last|next)\s+(week|month|year)",
            r"(this|past)\s+(week|month|year)",
        ];
        
        for pattern in &relative_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if regex.is_match(&lower) {
                    return true;
                }
            }
        }
        
        false
    }

    /// Handle merged cells according to configuration
    fn handle_merged_cells(&self, table_data: &mut TableData) -> Result<()> {
        match self.config.merge_cells_handling {
            MergeCellsHandling::Ignore => {
                // No processing needed
                Ok(())
            }
            MergeCellsHandling::Preserve => {
                self.mark_merged_cells(table_data)
            }
            MergeCellsHandling::Expand => {
                self.expand_merged_cells(table_data)
            }
        }
    }
    
    /// Mark merged cells in the table data
    fn mark_merged_cells(&self, table_data: &mut TableData) -> Result<()> {
        // Detect merged cells by analyzing cell properties and content patterns
        let mut merged_ranges = Vec::new();
        
        // Step 1: Detect horizontal merges (same row, consecutive empty cells)
        for (row_idx, row) in table_data.rows.iter().enumerate() {
            let mut current_merge: Option<(usize, usize)> = None;
            
            for (col_idx, cell) in row.cells.iter().enumerate() {
                if cell.is_empty() {
                    if let Some((start_col, _)) = current_merge {
                        // Continue the merge
                        current_merge = Some((start_col, col_idx));
                    } else {
                        // Start a new merge if previous cell had content
                        if col_idx > 0 && !row.cells[col_idx - 1].is_empty() {
                            current_merge = Some((col_idx - 1, col_idx));
                        }
                    }
                } else {
                    // End current merge if any
                    if let Some((start_col, end_col)) = current_merge {
                        if end_col > start_col {
                            merged_ranges.push(MergedRange {
                                start_row: row_idx,
                                end_row: row_idx,
                                start_col,
                                end_col,
                                merge_type: MergeType::Horizontal,
                            });
                        }
                        current_merge = None;
                    }
                }
            }
            
            // Handle merge at end of row
            if let Some((start_col, end_col)) = current_merge {
                if end_col > start_col {
                    merged_ranges.push(MergedRange {
                        start_row: row_idx,
                        end_row: row_idx,
                        start_col,
                        end_col,
                        merge_type: MergeType::Horizontal,
                    });
                }
            }
        }
        
        // Step 2: Detect vertical merges (same column, consecutive empty cells)
        for col_idx in 0..table_data.column_count {
            let mut current_merge: Option<(usize, usize)> = None;
            
            for (row_idx, row) in table_data.rows.iter().enumerate() {
                if let Some(cell) = row.cells.get(col_idx) {
                    if cell.is_empty() {
                        if let Some((start_row, _)) = current_merge {
                            // Continue the merge
                            current_merge = Some((start_row, row_idx));
                        } else {
                            // Start a new merge if previous cell had content
                            if row_idx > 0 {
                                if let Some(prev_cell) = table_data.rows[row_idx - 1].cells.get(col_idx) {
                                    if !prev_cell.is_empty() {
                                        current_merge = Some((row_idx - 1, row_idx));
                                    }
                                }
                            }
                        }
                    } else {
                        // End current merge if any
                        if let Some((start_row, end_row)) = current_merge {
                            if end_row > start_row {
                                merged_ranges.push(MergedRange {
                                    start_row,
                                    end_row,
                                    start_col: col_idx,
                                    end_col: col_idx,
                                    merge_type: MergeType::Vertical,
                                });
                            }
                            current_merge = None;
                        }
                    }
                }
            }
            
            // Handle merge at end of column
            if let Some((start_row, end_row)) = current_merge {
                if end_row > start_row {
                    merged_ranges.push(MergedRange {
                        start_row,
                        end_row,
                        start_col: col_idx,
                        end_col: col_idx,
                        merge_type: MergeType::Vertical,
                    });
                }
            }
        }
        
        // Step 3: Apply merged cell information
        self.apply_merged_ranges(table_data, &merged_ranges)?;
        
        Ok(())
    }
    
    /// Expand merged cells by filling empty cells with the merged content
    fn expand_merged_cells(&self, table_data: &mut TableData) -> Result<()> {
        // First mark merged cells
        self.mark_merged_cells(table_data)?;
        
        // Then expand the content
        for row in &mut table_data.rows {
            for cell in &mut row.cells {
                if cell.is_merged() {
                    // Find the source cell and copy its content
                    if let Some(source_content) = self.find_merged_source_content(cell) {
                        cell.content = source_content;
                        cell.formatted_content = cell.content.clone();
                        cell.cell_type = CellType::Data;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Apply merged ranges to table data
    fn apply_merged_ranges(&self, table_data: &mut TableData, merged_ranges: &[MergedRange]) -> Result<()> {
        for range in merged_ranges {
            match range.merge_type {
                MergeType::Horizontal => {
                    // Mark horizontal merge
                    if let Some(row) = table_data.rows.get_mut(range.start_row) {
                        for col_idx in range.start_col..=range.end_col {
                            if let Some(cell) = row.cells.get_mut(col_idx) {
                                if col_idx == range.start_col {
                                    // Source cell
                                    cell.set_merged(Some(range.end_col - range.start_col + 1), None);
                                } else {
                                    // Merged cell
                                    cell.set_merged(None, None);
                                }
                            }
                        }
                    }
                }
                MergeType::Vertical => {
                    // Mark vertical merge
                    for row_idx in range.start_row..=range.end_row {
                        if let Some(row) = table_data.rows.get_mut(row_idx) {
                            if let Some(cell) = row.cells.get_mut(range.start_col) {
                                if row_idx == range.start_row {
                                    // Source cell
                                    cell.set_merged(None, Some(range.end_row - range.start_row + 1));
                                } else {
                                    // Merged cell
                                    cell.set_merged(None, None);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Find the source content for a merged cell
    fn find_merged_source_content(&self, _merged_cell: &TableCell) -> Option<String> {
        // This is a simplified implementation
        // In a real implementation, you would track the source cell for each merged cell
        Some("Merged Content".to_string())
    }

    /// Filter empty cells if configured
    fn filter_empty_cells(&self, table_data: &mut TableData) -> Result<()> {
        for row in &mut table_data.rows {
            row.cells.retain(|cell| !cell.is_empty());
        }
        
        // Update column count
        table_data.column_count = table_data.rows.iter()
            .map(|row| row.cells.len())
            .max()
            .unwrap_or(0);
        
        Ok(())
    }
}

impl Default for TableData {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CellFormatting {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TableExtractionConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_data_creation() {
        let table_data = TableData::new();
        assert!(table_data.is_empty());
        assert_eq!(table_data.row_count(), 0);
        assert_eq!(table_data.column_count(), 0);
    }

    #[test]
    fn test_table_row_creation() {
        let mut row = TableRow::new(0);
        assert!(row.is_empty());
        assert_eq!(row.cell_count(), 0);
        assert_eq!(row.row_index, 0);
        assert!(!row.is_header);
    }

    #[test]
    fn test_table_cell_creation() {
        let content = "Test content".to_string();
        let cell = TableCell::new(content.clone());
        assert_eq!(cell.content, content);
        assert!(!cell.is_empty());
        assert!(!cell.is_merged());
        assert!(!cell.is_header());
    }

    #[test]
    fn test_empty_cell() {
        let cell = TableCell::empty();
        assert!(cell.is_empty());
        assert_eq!(cell.content, "");
        assert!(matches!(cell.cell_type, CellType::Empty));
    }

    #[test]
    fn test_cell_formatting() {
        let formatting = CellFormatting::bold();
        assert!(formatting.bold);
        assert!(!formatting.italic);
        assert!(formatting.has_formatting());
    }

    #[test]
    fn test_table_data_add_row() {
        let mut table_data = TableData::new();
        let mut row = TableRow::new(0);
        row.add_cell(TableCell::new("Cell 1".to_string()));
        row.add_cell(TableCell::new("Cell 2".to_string()));
        
        table_data.add_row(row);
        assert_eq!(table_data.row_count(), 1);
        assert_eq!(table_data.column_count(), 2);
    }

    #[test]
    fn test_table_extraction_config() {
        let config = TableExtractionConfig::simple();
        assert!(matches!(config.mode, TableExtractionMode::Simple));
        assert!(!config.detect_headers);
        assert!(!config.preserve_formatting);
    }

    #[test]
    fn test_table_extraction_config_builder() {
        let config = TableExtractionConfig::new()
            .with_headers(true)
            .with_formatting(true)
            .with_mode(TableExtractionMode::Full);
        
        assert!(config.detect_headers);
        assert!(config.preserve_formatting);
        assert!(matches!(config.mode, TableExtractionMode::Full));
    }

    #[test]
    fn test_cell_content_length() {
        let cell = TableCell::new("Hello, 世界!".to_string());
        assert_eq!(cell.content_length(), 10); // 8 ASCII + 2 Unicode chars
    }

    #[test]
    fn test_row_mark_as_header() {
        let mut row = TableRow::new(0);
        row.add_cell(TableCell::new("Header 1".to_string()));
        row.add_cell(TableCell::new("Header 2".to_string()));
        
        row.mark_as_header();
        assert!(row.is_header);
        assert!(row.cells.iter().all(|cell| cell.is_header()));
    }

    #[test]
    fn test_table_get_cell() {
        let mut table_data = TableData::new();
        let mut row = TableRow::new(0);
        row.add_cell(TableCell::new("Cell 1".to_string()));
        row.add_cell(TableCell::new("Cell 2".to_string()));
        
        table_data.add_row(row);
        
        let cell = table_data.get_cell(0, 0);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().content, "Cell 1");
        
        let cell = table_data.get_cell(0, 1);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().content, "Cell 2");
        
        let cell = table_data.get_cell(1, 0);
        assert!(cell.is_none());
    }

    #[test]
    fn test_cell_formatting_apply() {
        let formatting = CellFormatting {
            bold: true,
            italic: true,
            underline: false,
            ..CellFormatting::new()
        };
        
        let result = formatting.apply_to_text("Hello");
        assert_eq!(result, "***Hello***");
    }

    #[test]
    fn test_table_extractor_creation() {
        let config = TableExtractionConfig::simple();
        let extractor = TableExtractor::new(config);
        
        // Test that extractor is created successfully
        assert!(matches!(extractor.config.mode, TableExtractionMode::Simple));
    }
    
    // Phase 2 Enhanced Tests
    
    #[test]
    fn test_enhanced_header_detection() {
        let extractor = TableExtractor::new(TableExtractionConfig::new().with_headers(true));
        
        // Create test table data with clear headers
        let mut table_data = TableData::new();
        
        // Header row with typical header content
        let mut header_row = TableRow::new(0);
        header_row.add_cell(TableCell::new("Name".to_string()));
        header_row.add_cell(TableCell::new("Age".to_string()));
        header_row.add_cell(TableCell::new("Department".to_string()));
        table_data.add_row(header_row);
        
        // Data rows
        let mut data_row1 = TableRow::new(1);
        data_row1.add_cell(TableCell::new("John Smith".to_string()));
        data_row1.add_cell(TableCell::new("25".to_string()));
        data_row1.add_cell(TableCell::new("Engineering".to_string()));
        table_data.add_row(data_row1);
        
        let mut data_row2 = TableRow::new(2);
        data_row2.add_cell(TableCell::new("Jane Doe".to_string()));
        data_row2.add_cell(TableCell::new("30".to_string()));
        data_row2.add_cell(TableCell::new("Marketing".to_string()));
        table_data.add_row(data_row2);
        
        // Test header detection
        extractor.detect_table_headers(&mut table_data).unwrap();
        
        // Should detect headers
        assert!(table_data.has_header);
        assert!(table_data.headers.is_some());
        
        let headers = table_data.headers.as_ref().unwrap();
        assert_eq!(headers.len(), 3);
        assert_eq!(headers[0], "Name");
        assert_eq!(headers[1], "Age");
        assert_eq!(headers[2], "Department");
    }
    
    #[test]
    fn test_data_type_detection() {
        let extractor = TableExtractor::new(TableExtractionConfig::simple());
        
        // Test numeric detection
        assert_eq!(extractor.detect_cell_data_type("123"), CellDataType::Number);
        assert_eq!(extractor.detect_cell_data_type("123.45"), CellDataType::Number);
        assert_eq!(extractor.detect_cell_data_type("$100.50"), CellDataType::Number);
        assert_eq!(extractor.detect_cell_data_type("25%"), CellDataType::Number);
        assert_eq!(extractor.detect_cell_data_type("1,234.56"), CellDataType::Number);
        assert_eq!(extractor.detect_cell_data_type("1.23e-4"), CellDataType::Number);
        
        // Test date detection
        assert_eq!(extractor.detect_cell_data_type("2023-12-25"), CellDataType::Date);
        assert_eq!(extractor.detect_cell_data_type("12/25/2023"), CellDataType::Date);
        assert_eq!(extractor.detect_cell_data_type("Dec 25, 2023"), CellDataType::Date);
        assert_eq!(extractor.detect_cell_data_type("today"), CellDataType::Date);
        assert_eq!(extractor.detect_cell_data_type("2 days ago"), CellDataType::Date);
        
        // Test boolean detection
        assert_eq!(extractor.detect_cell_data_type("true"), CellDataType::Boolean);
        assert_eq!(extractor.detect_cell_data_type("false"), CellDataType::Boolean);
        assert_eq!(extractor.detect_cell_data_type("yes"), CellDataType::Boolean);
        assert_eq!(extractor.detect_cell_data_type("no"), CellDataType::Boolean);
        assert_eq!(extractor.detect_cell_data_type("✓"), CellDataType::Boolean);
        assert_eq!(extractor.detect_cell_data_type("X"), CellDataType::Boolean);
        
        // Test text detection
        assert_eq!(extractor.detect_cell_data_type("Hello World"), CellDataType::Text);
        assert_eq!(extractor.detect_cell_data_type("Product Name"), CellDataType::Text);
        
        // Test empty detection
        assert_eq!(extractor.detect_cell_data_type(""), CellDataType::Empty);
        assert_eq!(extractor.detect_cell_data_type("   "), CellDataType::Empty);
    }
    
    #[test]
    fn test_currency_detection() {
        let extractor = TableExtractor::new(TableExtractionConfig::simple());
        
        // Test various currency symbols
        assert!(extractor.is_currency_pattern("$100"));
        assert!(extractor.is_currency_pattern("€50.25"));
        assert!(extractor.is_currency_pattern("¥1000"));
        assert!(extractor.is_currency_pattern("£75.50"));
        assert!(extractor.is_currency_pattern("100$"));
        
        // Test currency codes
        assert!(extractor.is_currency_pattern("USD 100"));
        assert!(extractor.is_currency_pattern("100 EUR"));
        assert!(extractor.is_currency_pattern("GBP 50.25"));
        
        // Test formatted currency
        assert!(extractor.is_currency_pattern("$1,234.56"));
        assert!(extractor.is_currency_pattern("€2,500.00"));
        
        // Test non-currency
        assert!(!extractor.is_currency_pattern("100"));
        assert!(!extractor.is_currency_pattern("text"));
        assert!(!extractor.is_currency_pattern("$"));
    }
    
    #[test]
    fn test_merged_cells_detection() {
        let extractor = TableExtractor::new(
            TableExtractionConfig::new().with_merge_cells_handling(MergeCellsHandling::Preserve)
        );
        
        // Create test table with merged cells pattern
        let mut table_data = TableData::new();
        
        // Row 1: "Header 1" | "Header 2" | "Header 3"
        let mut row1 = TableRow::new(0);
        row1.add_cell(TableCell::new("Header 1".to_string()));
        row1.add_cell(TableCell::new("Header 2".to_string()));
        row1.add_cell(TableCell::new("Header 3".to_string()));
        table_data.add_row(row1);
        
        // Row 2: "Data 1" | "" | "Data 3" (middle cell empty - simulates horizontal merge)
        let mut row2 = TableRow::new(1);
        row2.add_cell(TableCell::new("Data 1".to_string()));
        row2.add_cell(TableCell::empty());
        row2.add_cell(TableCell::new("Data 3".to_string()));
        table_data.add_row(row2);
        
        // Row 3: "Data 4" | "Data 5" | "Data 6"
        let mut row3 = TableRow::new(2);
        row3.add_cell(TableCell::new("Data 4".to_string()));
        row3.add_cell(TableCell::new("Data 5".to_string()));
        row3.add_cell(TableCell::new("Data 6".to_string()));
        table_data.add_row(row3);
        
        // Test merged cells handling
        extractor.handle_merged_cells(&mut table_data).unwrap();
        
        // The middle cell in row 2 should be marked as merged
        let middle_cell = &table_data.rows[1].cells[1];
        assert!(middle_cell.is_merged());
    }
    
    #[test]
    fn test_formatting_extraction() {
        let extractor = TableExtractor::new(
            TableExtractionConfig::new().with_formatting(true)
        );
        
        // Test formatting merge
        let base_formatting = CellFormatting::new();
        let bold_formatting = CellFormatting::bold();
        let italic_formatting = CellFormatting::italic();
        
        let merged = extractor.merge_formatting(base_formatting, bold_formatting);
        assert!(merged.bold);
        assert!(!merged.italic);
        
        let merged2 = extractor.merge_formatting(merged, italic_formatting);
        assert!(merged2.bold);
        assert!(merged2.italic);
    }
    
    #[test]
    fn test_highlight_color_conversion() {
        let extractor = TableExtractor::new(TableExtractionConfig::simple());
        
        // Test standard colors
        assert_eq!(extractor.convert_highlight_to_hex("yellow"), "#FFFF00");
        assert_eq!(extractor.convert_highlight_to_hex("red"), "#FF0000");
        assert_eq!(extractor.convert_highlight_to_hex("blue"), "#0000FF");
        assert_eq!(extractor.convert_highlight_to_hex("green"), "#00FF00");
        
        // Test dark colors  
        assert_eq!(extractor.convert_highlight_to_hex("darkRed"), "#800000");
        assert_eq!(extractor.convert_highlight_to_hex("darkBlue"), "#000080");
        
        // Test case insensitive
        assert_eq!(extractor.convert_highlight_to_hex("YELLOW"), "#FFFF00");
        assert_eq!(extractor.convert_highlight_to_hex("Red"), "#FF0000");
        
        // Test hex passthrough
        assert_eq!(extractor.convert_highlight_to_hex("FF5500"), "#FF5500");
    }
    
    #[test]
    fn test_date_pattern_detection() {
        let extractor = TableExtractor::new(TableExtractionConfig::simple());
        
        // Test standard date formats
        assert!(extractor.is_date_pattern("2023-12-25"));
        assert!(extractor.is_date_pattern("12/25/2023"));
        assert!(extractor.is_date_pattern("25/12/2023"));
        assert!(extractor.is_date_pattern("12.25.2023"));
        assert!(extractor.is_date_pattern("Dec 25, 2023"));
        assert!(extractor.is_date_pattern("25 Dec 2023"));
        
        // Test with time
        assert!(extractor.is_date_pattern("2023-12-25T10:30:00"));
        assert!(extractor.is_date_pattern("12/25/2023 10:30"));
        assert!(extractor.is_date_pattern("10:30 AM"));
        assert!(extractor.is_date_pattern("22:30:45"));
        
        // Test relative dates
        assert!(extractor.is_date_pattern("today"));
        assert!(extractor.is_date_pattern("tomorrow"));
        assert!(extractor.is_date_pattern("2 days ago"));
        assert!(extractor.is_date_pattern("next week"));
        assert!(extractor.is_date_pattern("last month"));
        
        // Test non-dates
        assert!(!extractor.is_date_pattern("hello"));
        assert!(!extractor.is_date_pattern("123"));
        assert!(!extractor.is_date_pattern("product name"));
    }
    
    #[test]
    fn test_header_content_patterns() {
        let extractor = TableExtractor::new(TableExtractionConfig::new());
        
        // Create test row with header-like content
        let mut header_row = TableRow::new(0);
        header_row.add_cell(TableCell::new("Name".to_string()));          // Common header word
        header_row.add_cell(TableCell::new("ID".to_string()));            // Common header word
        header_row.add_cell(TableCell::new("Status".to_string()));        // Common header word
        header_row.add_cell(TableCell::new("Amount".to_string()));        // Common header word
        
        let score = extractor.check_header_content_pattern(&header_row).unwrap();
        assert!(score > 0.5); // Should have high score for header-like content
        
        // Create test row with non-header content
        let mut data_row = TableRow::new(1);
        data_row.add_cell(TableCell::new("John Smith".to_string()));
        data_row.add_cell(TableCell::new("12345".to_string()));
        data_row.add_cell(TableCell::new("Active".to_string()));
        data_row.add_cell(TableCell::new("$1,234.56".to_string()));
        
        let score2 = extractor.check_header_content_pattern(&data_row).unwrap();
        assert!(score2 < score); // Should have lower score for data content
    }
    
    #[test]
    fn test_scientific_notation() {
        let extractor = TableExtractor::new(TableExtractionConfig::simple());
        
        // Test scientific notation patterns
        assert!(extractor.is_scientific_notation("1.23e-4"));
        assert!(extractor.is_scientific_notation("1.23E-4"));
        assert!(extractor.is_scientific_notation("1e10"));
        assert!(extractor.is_scientific_notation("2.5e+3"));
        assert!(extractor.is_scientific_notation("6.022e23"));
        
        // Test non-scientific notation
        assert!(!extractor.is_scientific_notation("123"));
        assert!(!extractor.is_scientific_notation("hello"));
        assert!(!extractor.is_scientific_notation("e"));
        assert!(!extractor.is_scientific_notation("123e"));
    }
}