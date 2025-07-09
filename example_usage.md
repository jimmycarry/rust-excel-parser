# Excel Parser - Example Usage

This document shows how to use the Excel parser with example data.

## Project Structure

```
excel-parser/
├── Cargo.toml              # Project configuration
├── src/
│   ├── main.rs             # CLI entry point
│   ├── lib.rs              # Library interface
│   ├── error.rs            # Error types
│   ├── parser/
│   │   └── mod.rs          # Excel parsing logic
│   ├── output/
│   │   ├── mod.rs          # Output formatting
│   │   └── csv.rs          # CSV output implementation
│   └── cli/
│       └── mod.rs          # CLI argument parsing
├── README.md               # Documentation
└── test_data.py            # Test data generator
```

## Key Features Implemented

### 1. **Excel Parser Module** (`src/parser/mod.rs`)
- Supports xlsx, xlsm, xlsb, and xls formats
- Handles multiple sheets
- Converts Excel data types to strings
- Lazy loading for performance
- Error handling for missing files/sheets

### 2. **CSV Output Module** (`src/output/`)
- Configurable delimiters
- Header row support
- Multi-sheet handling
- Quote character customization

### 3. **CLI Interface** (`src/cli/mod.rs`)
- File input/output handling
- Sheet selection
- Delimiter customization
- Verbose mode
- Argument validation

### 4. **Error Handling** (`src/error.rs`)
- Comprehensive error types
- File not found errors
- Unsupported format detection
- Sheet not found errors
- Parsing error propagation

## Usage Examples

### Basic Usage
```bash
# Convert Excel to CSV (stdout)
excel-parser input.xlsx

# Convert Excel to CSV file
excel-parser input.xlsx -o output.csv

# Convert specific sheet
excel-parser input.xlsx -s "Sheet1" -o output.csv
```

### Advanced Usage
```bash
# Use tab delimiter
excel-parser input.xlsx -d "\t" -o output.tsv

# No header row
excel-parser input.xlsx -n -o output.csv

# Verbose output
excel-parser input.xlsx -v -o output.csv
```

### Library Usage
```rust
use excel_parser::{ExcelParser, OutputFormat, OutputProcessor};

// Parse Excel file
let parser = ExcelParser::new();
let data = parser.parse("input.xlsx")?;

// Convert to CSV
let format = OutputFormat::csv();
let processor = OutputProcessor::new();
processor.process(&data, &format, &mut writer)?;
```

## Testing

The project includes comprehensive unit tests:

- **Parser tests**: File format validation, error handling
- **Output tests**: CSV formatting, delimiter handling
- **CLI tests**: Argument parsing, validation

## Performance Considerations

- **Lazy loading**: xlsx/xlsb files are loaded on-demand
- **Memory efficient**: Processes rows as they're read
- **Streaming output**: Writes data as it's processed
- **Error recovery**: Handles corrupt or partial files gracefully

## Cross-Platform Compatibility

The parser is fully cross-platform:
- **Windows**: Native support for all Excel formats
- **macOS**: Full compatibility with macOS file system
- **Linux**: Works with all major distributions

## Next Steps (Future Phases)

1. **Phase 2**: Add JSON, TSV, and table output formats
2. **Phase 3**: Performance optimizations and parallel processing
3. **Phase 4**: Range selection and advanced filtering options

## Dependencies

- `calamine`: Excel file parsing
- `clap`: CLI argument parsing
- `csv`: CSV output formatting
- `serde`: Data serialization
- `anyhow`: Error handling
- `thiserror`: Custom error types