use excel_parser::{Args, ExcelParser, OutputProcessor, Result};
use std::fs::File;
use std::io::{self, BufWriter, Write};

fn main() -> Result<()> {
    let args = Args::parse_args();

    // Validate arguments
    if let Err(e) = args.validate() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    if args.verbose {
        eprintln!("Parsing file: {}", args.input.display());
    }

    // Create parser
    let parser = ExcelParser::new();

    // Parse Excel file
    let data = if let Some(sheet_name) = &args.sheet {
        // Parse specific sheet
        if args.verbose {
            eprintln!("Parsing sheet: {}", sheet_name);
        }
        let sheet = parser.parse_sheet(&args.input, sheet_name)?;
        excel_parser::parser::ExcelData {
            sheets: vec![sheet],
        }
    } else {
        // Parse all sheets
        if args.verbose {
            eprintln!("Parsing all sheets");
        }
        parser.parse(&args.input)?
    };

    if args.verbose {
        eprintln!("Found {} sheet(s)", data.sheets.len());
        for sheet in &data.sheets {
            eprintln!("  - {}: {} rows", sheet.name, sheet.data.len());
        }
    }

    // Create output format
    let format = args
        .get_output_format()
        .map_err(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        })
        .unwrap();

    // Create output processor
    let processor = OutputProcessor::new();

    // Write output
    match &args.output {
        Some(output_path) => {
            if args.verbose {
                eprintln!("Writing to file: {}", output_path.display());
            }
            let file = File::create(output_path)?;
            let mut writer = BufWriter::new(file);
            processor.process(&data, &format, &mut writer)?;
            writer.flush()?;
        }
        None => {
            if args.verbose {
                eprintln!("Writing to stdout");
            }
            let stdout = io::stdout();
            let mut writer = stdout.lock();
            processor.process(&data, &format, &mut writer)?;
            writer.flush()?;
        }
    }

    if args.verbose {
        eprintln!("Conversion completed successfully");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // Integration tests can be added here
    // For now, we'll rely on unit tests in individual modules
}
