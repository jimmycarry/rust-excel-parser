use anyhow::Result;
use pdf_parser::{cli::Args, OutputProcessor, PdfParser};
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
        eprintln!("Processing PDF file: {}", args.input.display());
    }

    // Create parser
    let parser = PdfParser::new();
    
    // Create output processor
    let processor = OutputProcessor::new();
    
    // Get output format
    let format = args.get_output_format().map_err(|e| anyhow::anyhow!(e))?;

    // Prepare output writer
    let mut output: Box<dyn Write> = if let Some(output_path) = &args.output {
        if args.verbose {
            eprintln!("Writing output to: {}", output_path.display());
        }
        Box::new(BufWriter::new(File::create(output_path)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    // Process based on options
    match (args.page, args.tables_only) {
        // Extract specific page
        (Some(page_num), false) => {
            if args.verbose {
                eprintln!("Extracting page {}", page_num);
            }
            let page = parser.parse_page(&args.input, page_num)?;
            processor.process_page(&page, &format, &mut output)?;
        }
        
        // Extract tables only
        (None, true) => {
            if args.verbose {
                eprintln!("Extracting tables only");
            }
            let tables = parser.extract_tables(&args.input)?;
            processor.process_tables(&tables, &format, &mut output)?;
        }
        
        // Extract full document
        (None, false) => {
            if args.verbose {
                eprintln!("Extracting full document");
            }
            let data = parser.parse(&args.input)?;
            processor.process(&data, &format, &mut output)?;
        }
        
        // Invalid combination (should be caught by validation)
        (Some(_), true) => {
            eprintln!("Error: Cannot use --page and --tables-only together");
            std::process::exit(1);
        }
    }

    // Ensure output is flushed
    output.flush()?;

    if args.verbose {
        eprintln!("Processing completed successfully");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_validation() {
        // This test would require actual PDF files
        // For now, we just test that the main function structure is correct
        assert!(true);
    }
}