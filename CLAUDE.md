# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a comprehensive document parser suite written in Rust that converts multiple document formats to various text formats. The project includes parsers for Excel files (.xlsx, .xlsm, .xlsb, .xls), Word documents (.doc, .docx), PDF files (.pdf), and PowerPoint presentations (.ppt, .pptx). The suite implements a modular architecture with separate concerns for parsing, output formatting, and CLI interface.

## Build and Development Commands

```bash
# Build development version
cargo build

# Build optimized release version
cargo build --release

# Run all tests
cargo test

# Run specific test module
cargo test parser::tests
cargo test output::json::tests

# Run with test output visible
cargo test -- --nocapture

# Format code
cargo fmt

# Lint code
cargo clippy

# Run the CLI
./target/debug/excel-parser file.xlsx -f json --pretty
./target/release/excel-parser file.xlsx -f table --max-width 100
./target/debug/doc-parser document.docx -f markdown --metadata
./target/release/pdf-parser document.pdf -f json --metadata --pretty
./target/debug/ppt-parser presentation.pptx -f html --metadata --css
./target/release/ppt-parser presentation.pptx -f json --metadata --pretty
```

## Architecture Overview

### Core Components

**Parser Module** (`src/parser/mod.rs`)
- `ExcelParser`: Main parsing engine using the `calamine` crate
- `ExcelData`: Represents parsed workbook with multiple sheets
- `Sheet`: Individual worksheet data with name and 2D string data
- Handles all Excel formats through `open_workbook_auto()`

**Output Module** (`src/output/`)
- `OutputFormat`: Enum defining CSV, JSON, and Table output types
- `OutputWriter`: Trait for implementing output formatters
- `OutputProcessor`: Orchestrates output formatting based on format type
- Modular design allows easy addition of new output formats

**CLI Module** (`src/cli/mod.rs`)
- `Args`: Clap-derived CLI argument structure
- Validates file existence, format support, and argument combinations
- Maps CLI options to `OutputFormat` configurations

**Error Module** (`src/error.rs`)
- `ExcelParserError`: Comprehensive error types for all failure modes
- Implements `From` traits for upstream library errors
- Custom `Result` type alias for convenience

### Key Design Patterns

**Trait-Based Output System**: The `OutputWriter` trait allows polymorphic output handling. New formats implement this trait and are registered in `OutputProcessor`.

**Error Propagation**: Uses `thiserror` for custom error types and `anyhow` for error context, with `?` operator throughout for clean error handling.

**CLI Validation Pipeline**: Arguments are parsed by clap, validated by `Args::validate()`, then converted to internal types via `Args::get_output_format()`.

## Output Format System

The parser suite supports multiple output formats through a unified interface:

### Excel Parser Formats
- **CSV**: Uses `csv` crate with configurable delimiters and headers
- **JSON**: Structured output with optional pretty-printing via `serde_json`  
- **Table**: Custom implementation with borders and width constraints

### DOC Parser Formats
- **Text**: Plain text extraction with formatting preservation
- **Markdown**: Structured markdown with YAML frontmatter
- **JSON**: Complete document structure with metadata

### PDF Parser Formats
- **Text**: Clean text extraction with page breaks
- **JSON**: Structured PDF data with metadata and tables
- **Markdown**: Document format with table preservation
- **CSV**: Table-only extraction for data analysis

### PPT Parser Formats
- **Text**: Slide-organized plain text extraction
- **JSON**: Complete presentation structure with metadata
- **Markdown**: Formatted slides with YAML frontmatter
- **HTML**: Web-ready output with CSS styling

Each format implements the `OutputWriter` trait and handles format-specific requirements.

## Testing Strategy

- **Unit tests** in each module test individual components
- **Integration-style tests** in CLI module validate argument parsing
- **Mock data** used in output format tests to avoid file dependencies
- Test files automatically cleaned up to avoid git pollution

## Key Dependencies

### Common Dependencies
- `clap`: CLI argument parsing with derive macros
- `serde`/`serde_json`: JSON serialization for structured output
- `thiserror`/`anyhow`: Error handling and propagation
- `tabled`: Table formatting utilities

### Excel Parser
- `calamine`: Excel file parsing (supports all major Excel formats)
- `csv`: Robust CSV output formatting

### DOC Parser
- `docx-rs`: DOCX file parsing with comprehensive support
- `dotext`: Legacy DOC format support

### PDF Parser
- `pdf-extract`: Text extraction from PDF documents
- `lopdf`: Low-level PDF document parsing
- `regex`: Pattern matching for table detection

### PPT Parser
- `zip`: ZIP file extraction for PPTX archives
- `quick-xml`: Fast XML parsing with namespace support
- `regex`: Content pattern detection and parsing
- `base64`: Encoding utilities for embedded content
- `chrono`: Date/time handling for metadata

## Performance Considerations

### Excel Parser
- Uses `calamine`'s lazy loading for large .xlsx/.xlsb files
- Streaming output architecture processes data as it's read
- Release build configured with LTO and strip for optimal binary size
- Memory-efficient: processes rows incrementally rather than loading entire files

### DOC Parser
- Multiple processing modes: Fast (text-only), Standard, Full (with metadata)
- Streaming output architecture for memory efficiency
- Batch processing with progress tracking and error recovery
- Optimized for both single-file and batch operations
- Memory-efficient: processes documents incrementally

### PDF Parser
- Advanced text extraction with page-level processing
- Intelligent table detection and extraction
- Comprehensive metadata extraction (title, author, creation date, etc.)
- Multiple output formats: Text, JSON, Markdown, CSV
- Memory-efficient: processes pages incrementally rather than loading entire documents

### PPT Parser
- ZIP + XML architecture for comprehensive PPTX parsing
- Slide-by-slide processing with content extraction
- Intelligent detection of tables, lists, and structured content
- Multiple output formats: Text, JSON, Markdown, HTML
- Memory-efficient: streaming XML processing with incremental slide parsing
- CSS-styled HTML output with responsive design

## Documentation

The doc-parser includes comprehensive documentation:

- **README.md**: Complete user guide with examples
- **DEVELOPER_GUIDE.md**: Architecture, development setup, and contribution guidelines
- **TROUBLESHOOTING.md**: Common issues and solutions
- **examples/**: Usage examples for both CLI and library
- **API Documentation**: Run `cargo doc --open` for detailed API docs

## DOC Parser Usage Examples

### Single File Processing
```bash
# Extract plain text
doc-parser document.docx

# Convert to JSON with metadata
doc-parser document.docx -f json --metadata --pretty

# Convert to Markdown
doc-parser document.docx -f markdown --metadata

# Save to file
doc-parser document.docx -o output.txt
```

### Batch Processing
```bash
# Process all DOCX files in current directory
doc-parser --batch . --output-dir ./converted -f json

# Process files matching a pattern
doc-parser --batch "*.docx" --output-dir ./output -f markdown

# Process with limits and verbose output
doc-parser --batch ./documents --max-files 10 -v --overwrite
```

### Performance Modes
```bash
# Fast text-only extraction
doc-parser document.docx --text-only

# Full parsing with metadata
doc-parser document.docx --metadata

# Preserve formatting
doc-parser document.docx --preserve-formatting
```

## PDF Parser Usage Examples

### Single File Processing
```bash
# Extract plain text
pdf-parser document.pdf

# Convert to JSON with metadata
pdf-parser document.pdf -f json --metadata --pretty

# Convert to Markdown
pdf-parser document.pdf -f markdown --metadata

# Extract specific page
pdf-parser document.pdf --page 3 -f text

# Extract tables only
pdf-parser document.pdf --tables-only -f csv

# Save to file
pdf-parser document.pdf -o output.txt
```

## PPT Parser Usage Examples

### Single File Processing
```bash
# Extract plain text
ppt-parser presentation.pptx

# Convert to JSON with metadata
ppt-parser presentation.pptx -f json --metadata --pretty

# Convert to Markdown with slide numbers
ppt-parser presentation.pptx -f markdown --metadata --slide-numbers

# Convert to HTML with CSS styling
ppt-parser presentation.pptx -f html --metadata --css

# Extract specific slide
ppt-parser presentation.pptx --slide 5 -f json --pretty

# Save to file
ppt-parser presentation.pptx -o output.md
```

## Development Workflow

### For Excel Parser
```bash
cd src/excel-parser
cargo test
cargo build
./target/debug/excel-parser sample.xlsx
```

### For DOC Parser  
```bash
cd src/doc-parser
cargo test
cargo build
./target/debug/doc-parser sample.docx
```

### For PDF Parser
```bash
cd src/pdf-parser
cargo test
cargo build
./target/debug/pdf-parser sample.pdf
```

### For PPT Parser
```bash
cd src/ppt-parser
cargo test
cargo build
./target/debug/ppt-parser sample.pptx
```

### Workspace Commands
```bash
# From workspace root
cargo test --all
cargo build --all
cargo clippy --all
cargo fmt --all
```

## Recent Updates

### Phase 4 Implementation (DOC Parser)
- ✅ Enhanced CLI support with batch processing
- ✅ Improved error handling with user-friendly messages
- ✅ Advanced batch processing with glob patterns
- ✅ Progress tracking and detailed logging
- ✅ Comprehensive documentation and examples
- ✅ Robust testing with 36+ passing tests

### Phase 5 Implementation (PDF Parser)
- ✅ Cross-platform PDF parsing with multiple output formats
- ✅ Intelligent table detection and extraction
- ✅ Comprehensive metadata extraction
- ✅ Page-level processing and selective extraction
- ✅ Memory-efficient streaming architecture
- ✅ Comprehensive testing with 22+ passing tests

### Phase 6 Implementation (PPT Parser)
- ✅ Comprehensive PowerPoint parsing with ZIP + XML architecture
- ✅ Multiple output formats: Text, JSON, Markdown, HTML
- ✅ Slide-level processing with specific slide extraction
- ✅ Intelligent content detection for tables and lists
- ✅ CSS-styled HTML output with responsive design
- ✅ Comprehensive testing with 34+ passing tests

### Key Features Added

#### PDF Parser Features
- PDF text extraction with page-level granularity
- Smart table detection using regex patterns
- Multiple output formats: Text, JSON, Markdown, CSV
- Metadata extraction (title, author, dates, page count)
- CLI interface consistent with other parsers
- Robust error handling and validation

#### PPT Parser Features
- PPTX parsing using ZIP file extraction and XML processing
- Slide text extraction with title and content separation
- Table and list detection with structured output
- HTML output with embedded CSS and responsive design
- Markdown output with YAML frontmatter and slide organization
- JSON output with comprehensive metadata and structured content
- Memory-efficient streaming architecture for large presentations
- Comprehensive CLI with slide-specific processing options