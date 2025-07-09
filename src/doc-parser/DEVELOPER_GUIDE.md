# Developer Guide

This guide provides comprehensive information for developers who want to contribute to or extend the doc-parser library.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Module Structure](#module-structure)
3. [Development Setup](#development-setup)
4. [Building and Testing](#building-and-testing)
5. [Code Style and Guidelines](#code-style-and-guidelines)
6. [Adding New Features](#adding-new-features)
7. [Extending Output Formats](#extending-output-formats)
8. [Performance Optimization](#performance-optimization)
9. [Error Handling](#error-handling)
10. [Testing Strategy](#testing-strategy)
11. [CI/CD Pipeline](#cicd-pipeline)
12. [Release Process](#release-process)

## Architecture Overview

The doc-parser is built with a modular architecture that separates concerns clearly:

```
┌─────────────────────────────────────────────────────────────┐
│                        CLI Layer                            │
│  ┌─────────────────────────────────────────────────────────┤
│  │ Command Line Interface (cli/mod.rs)                     │
│  │ - Argument parsing                                      │
│  │ - Batch processing logic                               │
│  │ - Input validation                                     │
│  └─────────────────────────────────────────────────────────┤
├─────────────────────────────────────────────────────────────┤
│                      Business Logic                         │
│  ┌─────────────────────────────────────────────────────────┤
│  │ Parser Layer (parser/mod.rs)                           │
│  │ - Document parsing coordination                        │
│  │ - Format detection                                     │
│  │ - Data structure management                            │
│  └─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┤
│  │ Output Layer (output/mod.rs)                           │
│  │ - Format conversion                                    │
│  │ - Output generation                                    │
│  │ - Writer abstraction                                   │
│  └─────────────────────────────────────────────────────────┤
├─────────────────────────────────────────────────────────────┤
│                    Implementation Layer                     │
│  ┌─────────────────────────────────────────────────────────┤
│  │ DOCX Parser (parser/docx.rs)                           │
│  │ - docx-rs integration                                  │
│  │ - Content extraction                                   │
│  │ - Structure analysis                                   │
│  └─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┤
│  │ DOC Parser (parser/doc.rs)                             │
│  │ - Legacy format support                               │
│  │ - Feature-gated implementation                        │
│  └─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┤
│  │ Text Processing (parser/text_extractor.rs)             │
│  │ - Text cleaning                                        │
│  │ - Formatting normalization                             │
│  └─────────────────────────────────────────────────────────┤
├─────────────────────────────────────────────────────────────┤
│                       Support Layer                        │
│  ┌─────────────────────────────────────────────────────────┤
│  │ Error Handling (error.rs)                              │
│  │ - Structured error types                              │
│  │ - User-friendly messages                              │
│  │ - Error conversion                                     │
│  └─────────────────────────────────────────────────────────┤
└─────────────────────────────────────────────────────────────┘
```

## Module Structure

### Core Modules

#### `parser/`
- **`mod.rs`**: Main parser interface and data structures
- **`docx.rs`**: DOCX file parsing using docx-rs
- **`doc.rs`**: Legacy DOC file parsing (feature-gated)
- **`text_extractor.rs`**: Text processing utilities

#### `output/`
- **`mod.rs`**: Output format coordination
- **`text.rs`**: Plain text output formatter
- **`json.rs`**: JSON output formatter
- **`markdown.rs`**: Markdown output formatter

#### `cli/`
- **`mod.rs`**: Command-line interface and argument parsing

#### `error.rs`
- Centralized error handling with structured error types

### Key Data Structures

```rust
// Core document representation
pub struct DocData {
    pub content: String,           // Original content
    pub metadata: DocMetadata,     // Document metadata
    pub sections: Vec<DocSection>, // Structured sections
    pub raw_text: String,          // Cleaned text
}

// Document metadata
pub struct DocMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub word_count: usize,
    pub paragraph_count: usize,
    // ... other metadata fields
}

// Document section
pub struct DocSection {
    pub section_type: SectionType,
    pub content: String,
    pub level: Option<u8>,
    pub formatting: Option<FormatInfo>,
}
```

## Development Setup

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- A code editor with Rust support (VS Code + rust-analyzer recommended)

### Environment Setup

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd rust-excel-parser/src/doc-parser
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Set up development tools**:
   ```bash
   # Install formatting and linting tools
   rustup component add rustfmt clippy
   
   # Install useful development tools
   cargo install cargo-watch cargo-expand
   ```

4. **VS Code setup** (recommended):
   - Install the "rust-analyzer" extension
   - Install the "Better TOML" extension
   - Configure settings in `.vscode/settings.json`:
     ```json
     {
       "rust-analyzer.cargo.features": ["docx"],
       "rust-analyzer.checkOnSave.command": "clippy"
     }
     ```

### Development Workflow

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Use cargo-watch for continuous testing**:
   ```bash
   cargo watch -x test
   ```

3. **Run specific tests**:
   ```bash
   cargo test parser::tests
   cargo test output::json::tests
   ```

4. **Check formatting and linting**:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

## Building and Testing

### Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Build with all features
cargo build --all-features

# Build documentation
cargo doc --open
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test parser::tests

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html

# Run integration tests
cargo test --test integration

# Run benchmarks
cargo bench
```

### Feature Flags

The project uses feature flags to control compilation:

```toml
[features]
default = ["docx"]
docx = ["docx-rs"]           # DOCX support (default)
legacy-doc = ["dotext"]      # Legacy DOC support
```

Build with specific features:
```bash
cargo build --features legacy-doc
cargo build --no-default-features --features docx
```

## Code Style and Guidelines

### Formatting

- Use `rustfmt` for consistent formatting
- Configure in `rustfmt.toml`:
  ```toml
  max_width = 100
  hard_tabs = false
  tab_spaces = 4
  ```

### Linting

- Use `clippy` for linting
- Address all clippy warnings
- Configure allowed lints in `lib.rs` if necessary

### Documentation

- **All public APIs must have documentation**
- Use doc comments (`///`) for public items
- Include examples in documentation:
  ```rust
  /// Parses a document and returns structured data.
  ///
  /// # Arguments
  ///
  /// * `file_path` - Path to the document file
  ///
  /// # Returns
  ///
  /// Returns `DocData` on success, or an error if parsing fails.
  ///
  /// # Example
  ///
  /// ```rust
  /// use doc_parser::DocParser;
  ///
  /// let parser = DocParser::new();
  /// let data = parser.parse("document.docx")?;
  /// ```
  pub fn parse<P: AsRef<Path>>(&self, file_path: P) -> Result<DocData>
  ```

### Error Handling

- Use structured error types from `error.rs`
- Provide user-friendly error messages
- Include context in error messages:
  ```rust
  // Good
  Err(DocParserError::FileNotFound {
      file: file_path.display().to_string(),
  })
  
  // Bad
  Err(DocParserError::Other("File not found".to_string()))
  ```

### Testing

- Write unit tests for all public functions
- Use descriptive test names
- Include both positive and negative test cases
- Use `tempfile` for temporary files in tests

### Performance

- Use `cargo bench` for performance testing
- Profile with `cargo flamegraph` for optimization
- Prefer streaming over loading entire files
- Use appropriate data structures for the use case

## Adding New Features

### 1. Planning

- Create an issue describing the feature
- Discuss the API design
- Consider backward compatibility
- Plan the implementation approach

### 2. Implementation Steps

1. **Add data structures** (if needed):
   ```rust
   // In parser/mod.rs
   pub struct NewFeatureData {
       pub field1: String,
       pub field2: Option<i32>,
   }
   ```

2. **Implement core logic**:
   ```rust
   // In appropriate module
   impl DocParser {
       pub fn new_feature_method(&self, input: &str) -> Result<NewFeatureData> {
           // Implementation
       }
   }
   ```

3. **Add tests**:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_new_feature() {
           // Test implementation
       }
   }
   ```

4. **Update documentation**:
   - Add doc comments
   - Update README if needed
   - Add examples

5. **Update CLI** (if needed):
   ```rust
   // In cli/mod.rs
   #[arg(long, help = "Enable new feature")]
   pub new_feature: bool,
   ```

### 3. Testing Checklist

- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Performance tests pass
- [ ] Documentation builds
- [ ] Examples work
- [ ] CLI help is updated

## Extending Output Formats

### Adding a New Output Format

1. **Create the formatter module**:
   ```rust
   // In output/new_format.rs
   use crate::error::Result;
   use crate::output::OutputWriter;
   use crate::parser::DocData;
   use std::io::Write;
   
   pub struct NewFormatWriter;
   
   impl OutputWriter for NewFormatWriter {
       fn write_document<W: Write>(&self, doc: &DocData, writer: &mut W) -> Result<()> {
           // Implementation
           Ok(())
       }
   }
   ```

2. **Add to OutputFormat enum**:
   ```rust
   // In output/mod.rs
   #[derive(Debug, Clone)]
   pub enum OutputFormat {
       // ... existing formats
       NewFormat {
           option1: bool,
           option2: String,
       },
   }
   ```

3. **Update OutputProcessor**:
   ```rust
   // In output/mod.rs
   impl OutputProcessor {
       pub fn process<W: Write>(&self, doc: &DocData, format: &OutputFormat, writer: &mut W) -> Result<()> {
           match format {
               // ... existing formats
               OutputFormat::NewFormat { option1, option2 } => {
                   let formatter = NewFormatWriter::new(*option1, option2.clone());
                   formatter.write_document(doc, writer)
               }
           }
       }
   }
   ```

4. **Add CLI support**:
   ```rust
   // In cli/mod.rs
   pub fn get_output_format(&self) -> Result<OutputFormat> {
       match self.format.to_lowercase().as_str() {
           // ... existing formats
           "newformat" => Ok(OutputFormat::NewFormat {
               option1: self.option1,
               option2: self.option2.clone(),
           }),
           // ...
       }
   }
   ```

5. **Add tests**:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_new_format_writer() {
           // Test implementation
       }
   }
   ```

## Performance Optimization

### Profiling

1. **Install profiling tools**:
   ```bash
   cargo install cargo-flamegraph
   cargo install cargo-instruments  # macOS only
   ```

2. **Generate flame graphs**:
   ```bash
   cargo flamegraph --bin doc-parser -- document.docx
   ```

3. **Use built-in benchmarks**:
   ```bash
   cargo bench
   ```

### Optimization Strategies

1. **Memory Management**:
   - Use `String::with_capacity()` for known sizes
   - Prefer `&str` over `String` where possible
   - Use `Vec::with_capacity()` for collections

2. **I/O Optimization**:
   - Use `BufReader` and `BufWriter`
   - Stream large files instead of loading entirely
   - Use memory-mapped files for very large documents

3. **Parsing Optimization**:
   - Parse only needed sections in fast mode
   - Cache expensive computations
   - Use lazy evaluation where appropriate

### Performance Testing

```rust
// Example benchmark
#[cfg(test)]
mod benchmarks {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn bench_parse_document(c: &mut Criterion) {
        let parser = DocParser::new();
        c.bench_function("parse_document", |b| {
            b.iter(|| {
                parser.parse(black_box("test.docx")).unwrap()
            })
        });
    }
    
    criterion_group!(benches, bench_parse_document);
    criterion_main!(benches);
}
```

## Error Handling

### Error Types

The library uses structured error types:

```rust
#[derive(Error, Debug)]
pub enum DocParserError {
    #[error("File not found: '{file}'")]
    FileNotFound { file: String },
    
    #[error("Unsupported format: '.{format}'")]
    UnsupportedFormat { format: String, file: String },
    
    // ... other error types
}
```

### Error Conversion

Implement `From` traits for external errors:

```rust
impl From<std::io::Error> for DocParserError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => DocParserError::FileNotFound {
                file: "unknown".to_string(),
            },
            _ => DocParserError::IoError {
                file: "unknown".to_string(),
                source: error,
            },
        }
    }
}
```

### User-Friendly Messages

Provide helpful error messages:

```rust
impl DocParserError {
    pub fn user_friendly_message(&self) -> String {
        match self {
            DocParserError::FileNotFound { file } => {
                format!("❌ File not found: '{}'\\n💡 Make sure the file path is correct", file)
            }
            // ... other error types
        }
    }
}
```

## Testing Strategy

### Test Categories

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test module interactions
3. **CLI Tests**: Test command-line interface
4. **Performance Tests**: Benchmark critical paths
5. **Property Tests**: Test with generated inputs

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    
    #[test]
    fn test_parse_valid_docx() {
        // Test with valid input
    }
    
    #[test]
    fn test_parse_invalid_file() {
        // Test error handling
    }
    
    #[test]
    fn test_parse_empty_file() {
        // Test edge cases
    }
}
```

### Test Data Management

- Use `tempfile` for temporary test files
- Create helper functions for test data
- Use const data for simple tests
- Clean up resources in tests

### Continuous Integration

Tests run automatically on:
- Pull requests
- Main branch pushes
- Scheduled nightly builds

## CI/CD Pipeline

### GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: cargo clippy -- -D warnings
      - run: cargo fmt --check
```

### Quality Gates

- All tests must pass
- Code coverage > 80%
- No clippy warnings
- Proper formatting
- Documentation builds

## Release Process

### Version Management

1. **Update version in `Cargo.toml`**
2. **Update `CHANGELOG.md`**
3. **Create release tag**:
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] Version bumped
- [ ] Examples tested
- [ ] Performance benchmarks run
- [ ] Release notes prepared

### Publishing

```bash
# Dry run
cargo publish --dry-run

# Publish to crates.io
cargo publish
```

## Contributing Guidelines

### Pull Request Process

1. **Fork and branch**
2. **Make changes**
3. **Add tests**
4. **Update documentation**
5. **Run checks**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```
6. **Submit PR**

### Code Review

- All PRs require review
- Address all feedback
- Keep PRs focused and small
- Write clear commit messages

### Issue Reporting

- Use issue templates
- Provide minimal reproduction cases
- Include environment information
- Label appropriately

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [docx-rs Documentation](https://docs.rs/docx-rs/)
- [clap Documentation](https://docs.rs/clap/)
- [serde Documentation](https://docs.rs/serde/)

---

This developer guide is a living document. Please update it as the project evolves and new patterns emerge.