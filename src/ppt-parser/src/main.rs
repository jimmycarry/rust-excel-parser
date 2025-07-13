use clap::Parser;
use ppt_parser::{Args, OutputProcessor, PptParser};
use std::fs::File;
use std::io::{self, BufWriter, Write};

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        print_error(&e, args.verbose);
        std::process::exit(1);
    }
}

fn run(args: &Args) -> ppt_parser::Result<()> {
    // Validate arguments
    args.validate()?;

    if args.verbose {
        eprintln!("Processing PPT file: {}", args.input_file.display());
        
        if let Some(slide_num) = args.slide {
            eprintln!("Extracting slide: {}", slide_num);
        } else {
            eprintln!("Processing all slides");
        }

        eprintln!("Output format: {:?}", args.format);
        
        if args.metadata {
            eprintln!("Including metadata");
        }
    }

    // Create parser
    let parser = PptParser::new();
    let output_format = args.get_output_format();
    let processor = OutputProcessor::new();

    // Create output writer
    let mut output: Box<dyn Write> = if let Some(output_file) = &args.output {
        Box::new(BufWriter::new(File::create(output_file)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    // Process based on whether we want a specific slide or all slides
    if let Some(slide_number) = args.slide {
        // Parse specific slide
        let slide = parser.parse_slide(&args.input_file, slide_number)?;
        processor.process_slide(&slide, &output_format, &mut output)?;
    } else {
        // Parse entire presentation
        let ppt_data = parser.parse(&args.input_file)?;
        processor.process(&ppt_data, &output_format, &mut output)?;
    }

    // Ensure all output is written
    output.flush()?;

    if args.verbose {
        eprintln!("Processing completed successfully");
    }

    Ok(())
}

fn print_error(error: &ppt_parser::PptParserError, verbose: bool) {
    match error {
        ppt_parser::PptParserError::FileNotFound(path) => {
            eprintln!("❌ File not found: '{}'", path);
            eprintln!("💡 Check the file path and try again");
        }
        ppt_parser::PptParserError::UnsupportedFormat(format) => {
            eprintln!("❌ Unsupported file format: '{}'", format);
            eprintln!("💡 Supported formats: .ppt, .pptx");
            eprintln!("💡 Try converting your file to PowerPoint format first");
        }
        ppt_parser::PptParserError::SlideNotFound(slide_num) => {
            eprintln!("❌ Slide not found: slide {}", slide_num);
            eprintln!("💡 Check the slide number and presentation content");
            eprintln!("💡 Use --verbose to see available slide range");
        }
        ppt_parser::PptParserError::InvalidSlideRange(msg) => {
            eprintln!("❌ Invalid slide range: {}", msg);
            eprintln!("💡 Slide numbers start from 1");
            eprintln!("💡 Use --slide option with valid slide number");
        }
        ppt_parser::PptParserError::EmptyPresentation => {
            eprintln!("❌ Empty presentation");
            eprintln!("💡 The presentation contains no slides");
            eprintln!("💡 Check if the file is corrupted or empty");
        }
        ppt_parser::PptParserError::ZipError(e) => {
            eprintln!("❌ ZIP extraction error: {}", e);
            eprintln!("💡 The PPTX file may be corrupted");
            eprintln!("💡 Try opening the file in PowerPoint to verify");
        }
        ppt_parser::PptParserError::XmlError(e) => {
            eprintln!("❌ XML parsing error: {}", e);
            eprintln!("💡 The presentation structure may be corrupted");
            eprintln!("💡 Try saving the file again in PowerPoint");
        }
        ppt_parser::PptParserError::ParsingError(msg) => {
            eprintln!("❌ Parsing error: {}", msg);
            eprintln!("💡 Check your command line arguments");
            eprintln!("💡 Use --help for usage information");
        }
        ppt_parser::PptParserError::MetadataError(msg) => {
            eprintln!("❌ Metadata extraction error: {}", msg);
            eprintln!("💡 The file metadata may be incomplete or corrupted");
            eprintln!("💡 Try running without --metadata flag");
        }
        ppt_parser::PptParserError::ContentError(msg) => {
            eprintln!("❌ Content extraction error: {}", msg);
            eprintln!("💡 Some slide content may be corrupted");
            eprintln!("💡 Try processing individual slides with --slide");
        }
        ppt_parser::PptParserError::IoError(e) => {
            eprintln!("❌ I/O error: {}", e);
            eprintln!("💡 Check file permissions and disk space");
        }
        ppt_parser::PptParserError::JsonError(e) => {
            eprintln!("❌ JSON serialization error: {}", e);
            eprintln!("💡 Try a different output format");
        }
        ppt_parser::PptParserError::Utf8Error(e) => {
            eprintln!("❌ UTF-8 encoding error: {}", e);
            eprintln!("💡 The presentation may contain invalid text encoding");
        }
        ppt_parser::PptParserError::InvalidXmlStructure(msg) => {
            eprintln!("❌ Invalid XML structure: {}", msg);
            eprintln!("💡 The presentation file may be corrupted");
            eprintln!("💡 Try re-saving the file in PowerPoint");
        }
    }

    if verbose {
        eprintln!("\nDetailed error information:");
        eprintln!("{:?}", error);
    }
}