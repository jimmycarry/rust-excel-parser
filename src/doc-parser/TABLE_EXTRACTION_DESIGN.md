# Word表格提取设计方案

## 📋 需求分析

### 当前状态
- 现有的`extract_table_text_simple`函数只返回占位符文本
- 需要实现完整的表格结构提取
- 支持多种输出格式和提取模式

### 目标功能
1. **结构化表格数据提取**
2. **多种输出格式支持**
3. **智能表头检测**
4. **单元格合并处理**
5. **格式信息保留**
6. **高性能处理**

## 🎯 设计方案

### 1. 数据结构设计

```rust
// 表格数据结构
#[derive(Debug, Clone)]
pub struct TableData {
    pub rows: Vec<TableRow>,
    pub headers: Option<Vec<String>>,
    pub has_header: bool,
    pub column_count: usize,
    pub row_count: usize,
    pub table_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub is_header: bool,
    pub row_index: usize,
}

#[derive(Debug, Clone)]
pub struct TableCell {
    pub content: String,
    pub formatted_content: String,
    pub colspan: Option<usize>,
    pub rowspan: Option<usize>,
    pub alignment: Option<CellAlignment>,
    pub formatting: Option<CellFormatting>,
    pub cell_type: CellType,
}

#[derive(Debug, Clone)]
pub enum CellAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone)]
pub enum CellType {
    Header,
    Data,
    Merged,
    Empty,
}

#[derive(Debug, Clone)]
pub struct CellFormatting {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub font_size: Option<u32>,
    pub font_family: Option<String>,
}
```

### 2. 提取模式设计

```rust
#[derive(Debug, Clone)]
pub enum TableExtractionMode {
    Simple,      // 简单文本提取
    Structured,  // 结构化数据提取
    Formatted,   // 格式化提取
    Full,        // 完整提取（包含所有信息）
}

#[derive(Debug, Clone)]
pub struct TableExtractionConfig {
    pub mode: TableExtractionMode,
    pub detect_headers: bool,
    pub preserve_formatting: bool,
    pub include_empty_cells: bool,
    pub merge_cells_handling: MergeCellsHandling,
    pub output_format: TableOutputFormat,
}

#[derive(Debug, Clone)]
pub enum MergeCellsHandling {
    Ignore,      // 忽略合并信息
    Preserve,    // 保留合并信息
    Expand,      // 展开合并的单元格
}

#[derive(Debug, Clone)]
pub enum TableOutputFormat {
    PlainText,
    CSV,
    TSV,
    Markdown,
    JSON,
    HTML,
}
```

### 3. 核心算法设计

#### 3.1 表格解析算法

```rust
impl TableExtractor {
    pub fn extract_table(&self, table: &docx_rs::Table, config: &TableExtractionConfig) -> Result<TableData> {
        let mut table_data = TableData::new();
        
        // Step 1: 基础数据提取
        for (row_index, row) in table.rows.iter().enumerate() {
            let mut table_row = TableRow::new(row_index);
            
            for (cell_index, cell) in row.cells.iter().enumerate() {
                let cell_data = self.extract_cell_data(cell, config)?;
                table_row.cells.push(cell_data);
            }
            
            table_data.rows.push(table_row);
        }
        
        // Step 2: 智能表头检测
        if config.detect_headers {
            self.detect_table_headers(&mut table_data)?;
        }
        
        // Step 3: 合并单元格处理
        self.handle_merged_cells(&mut table_data, config)?;
        
        // Step 4: 数据验证和清理
        self.validate_and_clean_table(&mut table_data)?;
        
        Ok(table_data)
    }
}
```

#### 3.2 智能表头检测算法

```rust
impl TableExtractor {
    fn detect_table_headers(&self, table_data: &mut TableData) -> Result<()> {
        if table_data.rows.is_empty() {
            return Ok(());
        }
        
        let first_row = &table_data.rows[0];
        
        // 检测策略1: 格式化差异
        let has_formatting_difference = self.check_formatting_difference(first_row, &table_data.rows[1..])?;
        
        // 检测策略2: 内容模式分析
        let has_header_pattern = self.check_header_content_pattern(first_row)?;
        
        // 检测策略3: 数据类型一致性
        let has_data_consistency = self.check_data_type_consistency(&table_data.rows[1..])?;
        
        // 综合判断
        if has_formatting_difference || has_header_pattern || has_data_consistency {
            table_data.has_header = true;
            table_data.headers = Some(first_row.cells.iter().map(|cell| cell.content.clone()).collect());
            table_data.rows[0].is_header = true;
        }
        
        Ok(())
    }
}
```

#### 3.3 单元格合并处理算法

```rust
impl TableExtractor {
    fn handle_merged_cells(&self, table_data: &mut TableData, config: &TableExtractionConfig) -> Result<()> {
        match config.merge_cells_handling {
            MergeCellsHandling::Ignore => {
                // 不处理合并信息
                Ok(())
            }
            MergeCellsHandling::Preserve => {
                // 保留合并信息，标记合并的单元格
                self.mark_merged_cells(table_data)
            }
            MergeCellsHandling::Expand => {
                // 展开合并的单元格
                self.expand_merged_cells(table_data)
            }
        }
    }
}
```

### 4. 输出格式化设计

```rust
impl TableFormatter {
    pub fn format_table(&self, table_data: &TableData, format: &TableOutputFormat) -> Result<String> {
        match format {
            TableOutputFormat::PlainText => self.format_as_plain_text(table_data),
            TableOutputFormat::CSV => self.format_as_csv(table_data),
            TableOutputFormat::TSV => self.format_as_tsv(table_data),
            TableOutputFormat::Markdown => self.format_as_markdown(table_data),
            TableOutputFormat::JSON => self.format_as_json(table_data),
            TableOutputFormat::HTML => self.format_as_html(table_data),
        }
    }
    
    fn format_as_markdown(&self, table_data: &TableData) -> Result<String> {
        let mut result = String::new();
        
        // 标题行
        if let Some(headers) = &table_data.headers {
            result.push_str("| ");
            for header in headers {
                result.push_str(&format!("{} | ", header));
            }
            result.push('\n');
            
            // 分隔线
            result.push_str("| ");
            for _ in headers {
                result.push_str("--- | ");
            }
            result.push('\n');
        }
        
        // 数据行
        for row in &table_data.rows {
            if table_data.has_header && row.is_header {
                continue; // 跳过标题行
            }
            
            result.push_str("| ");
            for cell in &row.cells {
                result.push_str(&format!("{} | ", cell.content));
            }
            result.push('\n');
        }
        
        Ok(result)
    }
}
```

### 5. 性能优化策略

#### 5.1 内存优化
- 使用`String`的`with_capacity`预分配内存
- 避免不必要的字符串复制
- 使用引用而非克隆

#### 5.2 处理优化
- 懒加载表格数据
- 并行处理大型表格
- 缓存计算结果

#### 5.3 算法优化
- 使用快速的表头检测算法
- 优化合并单元格处理
- 减少字符串操作

## 🔧 实现计划

### 阶段1: 基础数据结构
- [ ] 定义表格相关数据结构
- [ ] 实现基本的表格解析
- [ ] 添加单元测试

### 阶段2: 核心算法
- [ ] 实现表格内容提取
- [ ] 实现智能表头检测
- [ ] 实现合并单元格处理

### 阶段3: 输出格式化
- [ ] 实现多种输出格式
- [ ] 优化格式化算法
- [ ] 添加格式化测试

### 阶段4: 性能优化
- [ ] 内存使用优化
- [ ] 处理速度优化
- [ ] 大型表格处理优化

### 阶段5: 集成测试
- [ ] 端到端测试
- [ ] 性能基准测试
- [ ] 错误处理测试

## 📊 测试策略

### 单元测试
- 表格解析功能测试
- 表头检测算法测试
- 格式化函数测试
- 边界条件测试

### 集成测试
- 真实Word文档测试
- 复杂表格结构测试
- 性能压力测试
- 错误恢复测试

### 测试数据
- 简单表格
- 复杂表格（合并单元格）
- 嵌套表格
- 大型表格
- 损坏的表格

## 🎯 成功指标

### 功能指标
- [ ] 支持所有主要表格特性
- [ ] 95%以上的表头检测准确率
- [ ] 正确处理合并单元格
- [ ] 支持5种以上输出格式

### 性能指标
- [ ] 处理1000行表格 < 1秒
- [ ] 内存使用 < 100MB（大型表格）
- [ ] 支持并发处理
- [ ] 错误率 < 1%

### 质量指标
- [ ] 代码覆盖率 > 90%
- [ ] 所有测试通过
- [ ] 文档完整
- [ ] 用户友好的API

## 📝 API设计

### 公共接口
```rust
pub struct TableExtractor {
    config: TableExtractionConfig,
}

impl TableExtractor {
    pub fn new(config: TableExtractionConfig) -> Self;
    pub fn extract_table(&self, table: &docx_rs::Table) -> Result<TableData>;
    pub fn extract_all_tables(&self, docx: &docx_rs::Docx) -> Result<Vec<TableData>>;
    pub fn format_table(&self, table_data: &TableData, format: TableOutputFormat) -> Result<String>;
}
```

### 配置接口
```rust
impl TableExtractionConfig {
    pub fn new() -> Self;
    pub fn with_headers(mut self, detect: bool) -> Self;
    pub fn with_formatting(mut self, preserve: bool) -> Self;
    pub fn with_output_format(mut self, format: TableOutputFormat) -> Self;
    pub fn simple() -> Self;
    pub fn full() -> Self;
}
```

## 🚀 使用示例

```rust
// 简单使用
let config = TableExtractionConfig::simple();
let extractor = TableExtractor::new(config);
let table_data = extractor.extract_table(&docx_table)?;
let markdown = extractor.format_table(&table_data, TableOutputFormat::Markdown)?;

// 高级使用
let config = TableExtractionConfig::new()
    .with_headers(true)
    .with_formatting(true)
    .with_output_format(TableOutputFormat::JSON);
let extractor = TableExtractor::new(config);
let tables = extractor.extract_all_tables(&docx)?;
```

这个设计方案提供了完整的Word表格提取解决方案，支持多种模式和输出格式，具有良好的扩展性和性能。