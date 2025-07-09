# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a cross-platform Excel parser written in Rust that converts Excel files (.xlsx, .xlsm, .xlsb, .xls) to multiple text formats (CSV, JSON, Table). The project implements a modular architecture with separate concerns for parsing, output formatting, and CLI interface.

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

The parser supports three output formats through a unified interface:

- **CSV**: Uses `csv` crate with configurable delimiters and headers
- **JSON**: Structured output with optional pretty-printing via `serde_json`  
- **Table**: Custom implementation with borders and width constraints

Each format implements `OutputWriter` and handles both single sheets and multi-sheet workbooks.

## Testing Strategy

- **Unit tests** in each module test individual components
- **Integration-style tests** in CLI module validate argument parsing
- **Mock data** used in output format tests to avoid file dependencies
- Test files automatically cleaned up to avoid git pollution

## Key Dependencies

- `calamine`: Excel file parsing (supports all major Excel formats)
- `clap`: CLI argument parsing with derive macros
- `serde`/`serde_json`: JSON serialization for structured output
- `csv`: Robust CSV output formatting
- `tabled`: Table formatting utilities (though custom implementation used)
- `thiserror`/`anyhow`: Error handling and propagation

## Performance Considerations

- Uses `calamine`'s lazy loading for large .xlsx/.xlsb files
- Streaming output architecture processes data as it's read
- Release build configured with LTO and strip for optimal binary size
- Memory-efficient: processes rows incrementally rather than loading entire files