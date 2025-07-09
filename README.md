# Rust Document Parser Suite

A comprehensive collection of high-performance document parsers written in Rust. This workspace contains parsers for Excel spreadsheets and Microsoft Word documents, providing both command-line tools and library APIs.

## 🚀 Features

- **📊 Excel Parser**: Convert Excel files (.xlsx, .xlsm, .xlsb, .xls) to CSV, JSON, and Table formats
- **📄 DOC Parser**: Extract text, metadata, and structured data from Word documents (.doc, .docx)
- **🌍 Cross-platform**: Works on Windows, macOS, and Linux
- **⚡ High Performance**: Memory-efficient processing with streaming output
- **🔄 Batch Processing**: Process multiple files at once with progress tracking
- **🎯 Multiple Output Formats**: Choose from various output formats for different use cases
- **🛠️ CLI & Library**: Use as command-line tools or integrate into your Rust projects
- **💪 Robust Error Handling**: User-friendly error messages with helpful suggestions

## 📦 Workspace Structure

```
rust-excel-parser/
├── src/
│   ├── excel-parser/     # Excel file parser
│   └── doc-parser/       # Word document parser
├── Cargo.toml           # Workspace configuration
└── README.md           # This file
```

## 🏗️ Installation

### From Source

```bash
git clone https://github.com/jimmycarry/rust-excel-parser.git
cd rust-excel-parser
cargo build --release
```

The binaries will be available at:
- `target/release/excel-parser`
- `target/release/doc-parser`

### Build Individual Components

```bash
# Build Excel parser only
cargo build --release -p excel-parser

# Build DOC parser only
cargo build --release -p doc-parser

# Build all
cargo build --release
```

## 📊 Excel Parser

### Quick Start

```bash
# Convert Excel to CSV
./target/release/excel-parser data.xlsx -o output.csv

# Convert to JSON with pretty formatting
./target/release/excel-parser data.xlsx -f json --pretty -o output.json

# Convert to formatted table
./target/release/excel-parser data.xlsx -f table --max-width 100
```

### Supported Features

- **Input Formats**: .xlsx, .xlsm, .xlsb, .xls
- **Output Formats**: CSV, JSON, Table
- **Sheet Selection**: Process specific sheets or all sheets
- **Custom Delimiters**: Configure CSV output delimiters
- **Headers**: Optional header row handling

### Example Usage

```bash
# Basic conversion
excel-parser input.xlsx

# Specific sheet with custom delimiter
excel-parser data.xlsx -s "Sheet1" -d "|" -o output.csv

# JSON output with metadata
excel-parser data.xlsx -f json --pretty -o data.json

# Table format with custom width
excel-parser data.xlsx -f table --max-width 80
```

## 📄 DOC Parser

### Quick Start

```bash
# Extract plain text
./target/release/doc-parser document.docx

# Convert to JSON with metadata
./target/release/doc-parser document.docx -f json --metadata --pretty

# Convert to Markdown
./target/release/doc-parser document.docx -f markdown --metadata -o output.md
```

### Supported Features

- **Input Formats**: .docx (full support), .doc (basic support)
- **Output Formats**: Text, Markdown, JSON
- **Metadata Extraction**: Title, author, word count, etc.
- **Structured Content**: Headings, paragraphs, tables, lists
- **Batch Processing**: Process multiple files with glob patterns
- **Performance Modes**: Fast (text-only), Standard, Full (with metadata)

### Example Usage

```bash
# Basic text extraction
doc-parser document.docx

# Batch processing
doc-parser --batch ./documents --output-dir ./converted -f json

# Advanced batch processing with patterns
doc-parser --batch "*.docx" --output-dir ./output -f markdown --metadata

# Performance mode
doc-parser document.docx --text-only  # Fastest
doc-parser document.docx --metadata   # Full processing
```

## 🛠️ Library Usage

### Excel Parser Library

```rust
use excel_parser::{ExcelParser, OutputFormat, OutputProcessor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = ExcelParser::new();
    let data = parser.parse("input.xlsx")?;
    
    let format = OutputFormat::Json { pretty: true };
    let processor = OutputProcessor::new();
    processor.process(&data, &format, &mut std::io::stdout())?;
    
    Ok(())
}
```

### DOC Parser Library

```rust
use doc_parser::{DocParser, OutputFormat, OutputProcessor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = DocParser::new();
    let doc_data = parser.parse("document.docx")?;
    
    // Access metadata
    println!("Title: {:?}", doc_data.metadata.title);
    println!("Word count: {}", doc_data.metadata.word_count);
    
    // Convert to JSON
    let format = OutputFormat::Json { pretty: true, include_formatting: false };
    let processor = OutputProcessor::new();
    processor.process(&doc_data, &format, &mut std::io::stdout())?;
    
    Ok(())
}
```

## 🎯 Output Formats

### Excel Parser Formats

| Format | Description | Example |
|--------|-------------|---------|
| CSV | Comma-separated values | `data.csv` |
| JSON | Structured JSON data | `data.json` |
| Table | Formatted table display | Console output |

### DOC Parser Formats

| Format | Description | Features |
|--------|-------------|----------|
| Text | Plain text extraction | Line numbers, formatting preservation |
| Markdown | Structured markdown | YAML frontmatter, headings, tables |
| JSON | Complete document data | Metadata, sections, structure |

## 🔧 Command Line Options

### Excel Parser

```bash
excel-parser [OPTIONS] <INPUT_FILE>

Options:
  -o, --output <OUTPUT>          Output file path
  -s, --sheet <SHEET>            Specific sheet name
  -f, --format <FORMAT>          Output format: csv, json, table
  -d, --delimiter <DELIMITER>    CSV delimiter [default: ,]
      --pretty                   Pretty print JSON
      --max-width <WIDTH>        Table max width
  -v, --verbose                  Enable verbose output
  -h, --help                     Print help
```

### DOC Parser

```bash
doc-parser [OPTIONS] <INPUT>

Options:
  -o, --output <OUTPUT>          Output file path
  -f, --format <FORMAT>          Output format: text, markdown, json
  -b, --batch <PATTERN>          Batch processing pattern
      --output-dir <DIR>         Output directory for batch
      --metadata                 Include metadata
      --text-only                Fast text-only mode
      --pretty                   Pretty print JSON
      --line-numbers             Add line numbers
      --overwrite                Overwrite existing files
      --max-files <N>            Max files to process
  -v, --verbose                  Enable verbose output
  -h, --help                     Print help
```

## 🚀 Performance

### Excel Parser
- Lazy loading for large .xlsx/.xlsb files
- Memory-efficient row-by-row processing
- Streaming output for large datasets
- Optimized for both single sheets and workbooks

### DOC Parser
- Multiple processing modes (Fast, Standard, Full)
- Batch processing with progress tracking
- Memory-efficient document parsing
- Streaming output architecture

### Benchmarks

| Operation | File Size | Processing Time | Memory Usage |
|-----------|-----------|-----------------|--------------|
| Excel to CSV | 10MB | ~2s | ~50MB |
| Excel to JSON | 10MB | ~3s | ~60MB |
| DOCX to Text | 1MB | ~50ms | ~10MB |
| DOCX to JSON | 1MB | ~150ms | ~20MB |

## 🛡️ Error Handling

Both parsers provide comprehensive error handling:

```bash
# Excel Parser
❌ File not found: 'missing.xlsx'
💡 Check the file path and try again

# DOC Parser
❌ Unsupported file format: '.txt'
💡 Supported formats: .doc, .docx
💡 Try converting your file to Word format first
```

## 📚 Documentation

### Excel Parser
- See `src/excel-parser/README.md` for detailed usage
- API documentation: `cargo doc --open -p excel-parser`

### DOC Parser
- See `src/doc-parser/README.md` for complete guide
- Developer guide: `src/doc-parser/DEVELOPER_GUIDE.md`
- Troubleshooting: `src/doc-parser/TROUBLESHOOTING.md`
- API documentation: `cargo doc --open -p doc-parser`

## 🧪 Testing

```bash
# Run all tests
cargo test

# Test specific parser
cargo test -p excel-parser
cargo test -p doc-parser

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## 🔄 Development

### Setup

```bash
# Install dependencies
cargo build

# Format code
cargo fmt

# Run linter
cargo clippy

# Generate documentation
cargo doc --open
```

### Workspace Commands

```bash
# Build all parsers
cargo build --all

# Test all parsers
cargo test --all

# Check all parsers
cargo clippy --all
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust best practices
- Add tests for new features
- Update documentation
- Use `cargo fmt` and `cargo clippy`
- Write clear commit messages

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

### Excel Parser
- Built with [calamine](https://github.com/tafia/calamine) for Excel parsing
- Uses [clap](https://github.com/clap-rs/clap) for CLI interface
- CSV output powered by [csv](https://github.com/BurntSushi/rust-csv)

### DOC Parser
- Built with [docx-rs](https://github.com/PoiScript/docx-rs) for DOCX parsing
- Uses [dotext](https://github.com/anvie/dotext) for legacy DOC support
- Error handling with [thiserror](https://github.com/dtolnay/thiserror)
- JSON serialization with [serde](https://github.com/serde-rs/serde)

## 📊 Project Stats

- **Languages**: Rust
- **Parsers**: 2 (Excel, DOC/DOCX)
- **Output Formats**: 5 (CSV, JSON, Table, Text, Markdown)
- **Test Coverage**: 36+ tests
- **Documentation**: Comprehensive guides and examples
- **Cross-platform**: Windows, macOS, Linux

## 🗺️ Roadmap

### Excel Parser
- [ ] Add XML output format
- [ ] Support for Excel formulas
- [ ] Conditional formatting extraction
- [ ] Chart data extraction

### DOC Parser
- [ ] Enhanced table parsing
- [ ] Image extraction
- [ ] Advanced formatting preservation
- [ ] PowerPoint support (.pptx)

### General
- [ ] Web interface
- [ ] Docker containers
- [ ] Cloud deployment guides
- [ ] Performance optimizations

---

**Built with ❤️ in Rust**