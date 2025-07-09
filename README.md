# Excel Parser

A cross-platform Excel parser written in Rust that converts Excel files to text formats.

## Features

- **Cross-platform**: Works on Windows, macOS, and Linux
- **Multiple formats**: Supports .xlsx, .xlsm, .xlsb, and .xls files
- **CSV output**: Converts Excel data to CSV format
- **Command-line interface**: Easy to use from the command line
- **Single or multiple sheets**: Process specific sheets or all sheets
- **Customizable**: Custom delimiters and header options

## Installation

### From Source

```bash
git clone https://github.com/example/excel-parser.git
cd excel-parser
cargo build --release
```

The binary will be available at `target/release/excel-parser`.

## Usage

### Basic Usage

```bash
# Convert Excel file to CSV (output to stdout)
excel-parser input.xlsx

# Convert Excel file to CSV (output to file)
excel-parser input.xlsx -o output.csv

# Convert specific sheet
excel-parser input.xlsx -s "Sheet1" -o output.csv

# Use custom delimiter (tab-separated)
excel-parser input.xlsx -d "\t" -o output.tsv

# Don't treat first row as header
excel-parser input.xlsx -n -o output.csv

# Verbose output
excel-parser input.xlsx -v -o output.csv
```

### Command Line Options

```
excel-parser [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Input Excel file (.xlsx, .xlsm, .xlsb, .xls)

Options:
  -o, --output <OUTPUT>      Output file path (default: stdout)
  -s, --sheet <SHEET>        Specific sheet name to process
  -d, --delimiter <DELIMITER> Custom delimiter for CSV output [default: ,]
  -n, --no-header            Don't treat first row as header
  -v, --verbose              Enable verbose output
  -h, --help                 Print help information
  -V, --version              Print version information
```

### Examples

```bash
# Convert entire workbook to CSV
excel-parser data.xlsx > output.csv

# Convert specific sheet with custom delimiter
excel-parser data.xlsx -s "Sales Data" -d "|" -o sales.csv

# Convert without headers using tab delimiter
excel-parser data.xlsx -n -d "\t" -o data.tsv

# Process with verbose output
excel-parser large_file.xlsx -v -o processed_data.csv
```

## Library Usage

You can also use this as a Rust library:

```rust
use excel_parser::{ExcelParser, OutputFormat, OutputProcessor};
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create parser
    let parser = ExcelParser::new();
    
    // Parse Excel file
    let data = parser.parse("input.xlsx")?;
    
    // Create output format
    let format = OutputFormat::csv();
    
    // Process and write to file
    let file = File::create("output.csv")?;
    let mut writer = BufWriter::new(file);
    
    let processor = OutputProcessor::new();
    processor.process(&data, &format, &mut writer)?;
    
    Ok(())
}
```

## Supported Formats

### Input Formats
- `.xlsx` - Excel 2007+ format
- `.xlsm` - Excel 2007+ format with macros
- `.xlsb` - Excel 2007+ binary format
- `.xls` - Excel 97-2003 format

### Output Formats
- CSV (Comma-Separated Values)
- TSV (Tab-Separated Values)
- Custom delimiter-separated values

## Performance

The parser is optimized for performance:
- Lazy loading for .xlsx and .xlsb files
- Memory-efficient processing
- Streaming output for large files

## Error Handling

The parser provides detailed error messages for:
- File not found
- Unsupported file formats
- Parsing errors
- Missing sheets
- Invalid ranges

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [calamine](https://github.com/tafia/calamine) for Excel parsing
- Uses [clap](https://github.com/clap-rs/clap) for CLI interface
- CSV output powered by [csv](https://github.com/BurntSushi/rust-csv)