# Doc Parser

A high-performance, cross-platform document parser for Microsoft Word documents (DOC/DOCX) written in Rust. Extract text content, metadata, and structured data from Word documents with support for multiple output formats.

## Features

- 🚀 **High Performance**: Built with Rust for speed and memory efficiency
- 📄 **Multiple Formats**: Support for both legacy DOC and modern DOCX files
- 🎯 **Flexible Output**: Export to Text, Markdown, and JSON formats
- 🔄 **Batch Processing**: Process multiple files at once with directory scanning
- 📊 **Rich Metadata**: Extract document properties, word counts, and structure
- 🛠️ **CLI & Library**: Use as a command-line tool or integrate as a Rust library
- 🌍 **Cross-Platform**: Works on Windows, macOS, and Linux
- 💪 **Robust Error Handling**: User-friendly error messages with helpful suggestions

## Installation

### From Source

```bash
git clone <repository-url>
cd rust-excel-parser/src/doc-parser
cargo build --release
```

The binary will be available at `target/release/doc-parser`.

### Using Cargo

```bash
cargo install --path .
```

## Quick Start

### Basic Usage

```bash
# Extract text from a DOCX file
doc-parser document.docx

# Save output to a file
doc-parser document.docx -o output.txt

# Export as JSON with metadata
doc-parser document.docx -f json --metadata --pretty
```

### Batch Processing

```bash
# Process all DOCX files in current directory
doc-parser --batch . --output-dir ./converted -f markdown

# Process files matching a pattern
doc-parser --batch "*.docx" --output-dir ./output -f json --pretty

# Limit processing to 10 files maximum
doc-parser --batch ./docs --max-files 10 -f text
```

## Command Line Options

### Basic Options

| Option | Description | Default |
|--------|-------------|---------|
| `<INPUT>` | Input DOC/DOCX file path | Required |
| `-o, --output <FILE>` | Output file path | stdout |
| `-f, --format <FORMAT>` | Output format: text, markdown, json | text |
| `-v, --verbose` | Enable verbose output | false |

### Content Options

| Option | Description |
|--------|-------------|
| `--metadata` | Include document metadata in output |
| `--preserve-formatting` | Preserve text formatting (where supported) |
| `--text-only` | Extract only plain text (fastest mode) |
| `--line-numbers` | Add line numbers to text output |
| `--pretty` | Pretty print JSON output |

### Batch Processing Options

| Option | Description |
|--------|-------------|
| `-b, --batch <PATTERN>` | Process multiple files from directory or glob pattern |
| `--output-dir <DIR>` | Output directory for batch processing |
| `--overwrite` | Overwrite existing output files |
| `--max-files <N>` | Maximum number of files to process |

## Output Formats

### Text Format

Plain text extraction with optional formatting preservation:

```bash
doc-parser document.docx -f text --line-numbers
```

### Markdown Format

Structured markdown with YAML frontmatter:

```bash
doc-parser document.docx -f markdown --metadata
```

Output example:
```markdown
---
title: "Document Title"
author: "Author Name"
word_count: 1234
---

# Heading 1

This is a paragraph with **bold** text.

## Heading 2

- List item 1
- List item 2
```

### JSON Format

Structured JSON with full document data:

```bash
doc-parser document.docx -f json --pretty --metadata
```

Output example:
```json
{
  "metadata": {
    "title": "Document Title",
    "author": "Author Name",
    "word_count": 1234,
    "paragraph_count": 56,
    "character_count": 7890
  },
  "content": "Full document text...",
  "sections": [
    {
      "type": "heading",
      "level": 1,
      "content": "Heading 1"
    },
    {
      "type": "paragraph",
      "content": "This is a paragraph..."
    }
  ]
}
```

## Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
doc-parser = { path = "../doc-parser" }
```

### Basic Example

```rust
use doc_parser::{DocParser, OutputFormat, OutputProcessor};
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create parser
    let parser = DocParser::new();
    
    // Parse document
    let doc_data = parser.parse("document.docx")?;
    
    // Extract just text
    let text = parser.extract_text("document.docx")?;
    println!("Text: {}", text);
    
    // Convert to JSON
    let format = OutputFormat::Json {
        pretty: true,
        include_formatting: false,
    };
    
    let processor = OutputProcessor::new();
    let mut output = Vec::new();
    processor.process(&doc_data, &format, &mut output)?;
    
    println!("JSON: {}", String::from_utf8(output)?);
    
    Ok(())
}
```

### Advanced Example

```rust
use doc_parser::{DocParser, DocData, DocMetadata, DocSection};

fn analyze_document(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parser = DocParser::new();
    let doc_data = parser.parse(path)?;
    
    // Analyze metadata
    println!("Document Analysis:");
    println!("- Title: {:?}", doc_data.metadata.title);
    println!("- Word Count: {}", doc_data.metadata.word_count);
    println!("- Paragraphs: {}", doc_data.metadata.paragraph_count);
    
    // Analyze structure
    let headings: Vec<_> = doc_data.sections
        .iter()
        .filter_map(|section| match &section.section_type {
            doc_parser::parser::SectionType::Heading(level) => {
                Some((level, &section.content))
            }
            _ => None,
        })
        .collect();
    
    println!("Document Structure:");
    for (level, content) in headings {
        println!("  {}: {}", "  ".repeat(*level as usize), content);
    }
    
    Ok(())
}
```

## Error Handling

The parser provides detailed error messages with helpful suggestions:

```bash
$ doc-parser nonexistent.docx
❌ File not found: 'nonexistent.docx'
💡 Make sure the file path is correct and the file exists.

$ doc-parser document.txt
❌ Unsupported file format: '.txt'
💡 Supported formats: .doc, .docx
💡 Try converting your file to Word format first.
```

## Performance

### Processing Modes

- **Fast Mode** (`--text-only`): Extract only plain text, fastest performance
- **Standard Mode**: Extract text with basic structure
- **Full Mode** (`--metadata`): Extract everything including metadata

### Benchmarks

| File Size | Mode | Processing Time | Memory Usage |
|-----------|------|----------------|--------------|
| 1MB DOCX | Fast | ~50ms | ~10MB |
| 1MB DOCX | Standard | ~100ms | ~15MB |
| 1MB DOCX | Full | ~150ms | ~20MB |

### Batch Processing

- Parallel processing of multiple files
- Progress reporting and error recovery
- Memory-efficient streaming for large files

## Supported Features

### Document Elements

- ✅ Paragraphs and text runs
- ✅ Headings (1-6 levels)
- ✅ Tables (basic structure)
- ✅ Lists (ordered and unordered)
- ✅ Hyperlinks
- ✅ Basic text formatting (bold, italic, underline)
- ⚠️ Images (placeholder extraction)
- ⚠️ Headers and footers (basic support)

### Metadata

- ✅ Document title
- ✅ Author information
- ✅ Word/paragraph/character counts
- ✅ Creation and modification dates
- ✅ Subject and keywords
- ⚠️ Custom properties (limited support)

### File Formats

- ✅ **DOCX**: Full support (Office 2007+)
- ⚠️ **DOC**: Basic support (requires `legacy-doc` feature)

## Troubleshooting

### Common Issues

**"File not found" error**
- Check file path is correct
- Ensure file exists and is accessible
- Use absolute paths if relative paths don't work

**"Unsupported format" error**
- Only DOC and DOCX files are supported
- Convert other formats to Word format first
- Check file extension matches actual format

**"Failed to parse" error**
- File may be corrupted
- Try opening in Microsoft Word to verify
- Check if file is password-protected

**Memory issues with large files**
- Use `--text-only` mode for faster processing
- Process files in smaller batches
- Increase system memory if needed

### Debugging

Enable verbose output for detailed information:

```bash
doc-parser document.docx -v
```

For library usage, errors implement the standard `Error` trait:

```rust
match parser.parse("document.docx") {
    Ok(doc_data) => { /* process */ }
    Err(e) => {
        eprintln!("Error: {}", e);
        eprintln!("Debug: {:?}", e);
    }
}
```

## Development

### Building from Source

```bash
git clone <repository-url>
cd rust-excel-parser/src/doc-parser
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test parser::tests

# Run with output
cargo test -- --nocapture
```

### Features

```bash
# Enable legacy DOC support
cargo build --features legacy-doc

# Default features (DOCX only)
cargo build
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Run linter (`cargo clippy`)
- Add documentation for public APIs
- Include tests for new features

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Changelog

### Version 0.1.0

- Initial release
- DOCX parsing support
- Text, Markdown, and JSON output formats
- Batch processing capabilities
- Enhanced error handling
- Command-line interface
- Rust library API

## Acknowledgments

- Built with [docx-rs](https://github.com/PoiScript/docx-rs) for DOCX parsing
- Uses [clap](https://github.com/clap-rs/clap) for CLI interface
- JSON serialization with [serde](https://github.com/serde-rs/serde)
- Error handling with [thiserror](https://github.com/dtolnay/thiserror)