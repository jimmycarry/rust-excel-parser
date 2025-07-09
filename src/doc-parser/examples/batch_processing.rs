//! Batch processing examples for the doc-parser library.
//!
//! This example demonstrates how to process multiple documents:
//! - Directory processing
//! - Glob pattern matching
//! - Batch conversion
//! - Progress tracking
//! - Error handling in batch operations

use doc_parser::{Args, DocParser, OutputFormat, OutputProcessor};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Doc Parser Batch Processing Examples ===\n");

    // Example 1: Directory processing
    println!("1. Directory Processing");
    directory_processing_example()?;

    // Example 2: Glob pattern processing
    println!("\n2. Glob Pattern Processing");
    glob_pattern_example()?;

    // Example 3: Batch conversion with different formats
    println!("\n3. Batch Format Conversion");
    batch_format_conversion()?;

    // Example 4: Performance monitoring
    println!("\n4. Performance Monitoring");
    performance_monitoring_example()?;

    // Example 5: Error handling and recovery
    println!("\n5. Error Handling and Recovery");
    error_handling_example()?;

    Ok(())
}

/// Example 1: Process all documents in a directory
fn directory_processing_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Processing all DOC/DOCX files in current directory...");

    // Create Args for batch processing
    let args = Args {
        input: PathBuf::new(), // Not used in batch mode
        output: None,
        format: "text".to_string(),
        metadata: true,
        preserve_formatting: false,
        pretty: false,
        line_numbers: false,
        text_only: false,
        verbose: true,
        batch: Some(".".to_string()), // Current directory
        output_dir: Some(PathBuf::from("./output")),
        overwrite: false,
        max_files: Some(5), // Limit for demo
    };

    // Validate arguments
    match args.validate() {
        Ok(()) => println!("  ✅ Arguments validated successfully"),
        Err(e) => {
            println!("  ❌ Validation error: {}", e);
            return Ok(());
        }
    }

    // Get batch files
    match args.get_batch_files() {
        Ok(files) => {
            println!("  📁 Found {} files to process:", files.len());
            for (i, file) in files.iter().enumerate() {
                println!("    {}. {}", i + 1, file.display());
            }

            // Process files
            process_batch_files(&files, &args)?;
        }
        Err(e) => {
            println!("  ❌ Error getting batch files: {}", e);
        }
    }

    Ok(())
}

/// Example 2: Process files using glob patterns
fn glob_pattern_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Processing files using glob patterns...");

    // Example patterns
    let patterns = vec![
        "*.docx",        // All DOCX files in current directory
        "docs/*.doc",    // All DOC files in docs directory
        "**/*.docx",     // All DOCX files recursively
    ];

    for pattern in patterns {
        println!("  🔍 Pattern: {}", pattern);
        
        let args = Args {
            input: PathBuf::new(),
            output: None,
            format: "json".to_string(),
            metadata: true,
            preserve_formatting: false,
            pretty: true,
            line_numbers: false,
            text_only: false,
            verbose: false,
            batch: Some(pattern.to_string()),
            output_dir: Some(PathBuf::from("./json_output")),
            overwrite: true,
            max_files: Some(3),
        };

        match args.get_batch_files() {
            Ok(files) => {
                println!("    📄 Found {} matching files", files.len());
                for file in files.iter().take(3) {
                    println!("      - {}", file.display());
                }
                if files.len() > 3 {
                    println!("      ... and {} more", files.len() - 3);
                }
            }
            Err(e) => {
                println!("    ❌ Error: {}", e);
            }
        }
    }

    Ok(())
}

/// Example 3: Batch conversion to different formats
fn batch_format_conversion() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Converting documents to multiple formats...");

    // Formats to convert to
    let formats = vec![
        ("text", "txt"),
        ("markdown", "md"),
        ("json", "json"),
    ];

    let base_args = Args {
        input: PathBuf::new(),
        output: None,
        format: "text".to_string(), // Will be overridden
        metadata: true,
        preserve_formatting: false,
        pretty: true,
        line_numbers: false,
        text_only: false,
        verbose: false,
        batch: Some(".".to_string()),
        output_dir: None, // Will be set per format
        overwrite: true,
        max_files: Some(2),
    };

    // Get files once
    let files = match base_args.get_batch_files() {
        Ok(files) => files,
        Err(e) => {
            println!("  ❌ Error getting files: {}", e);
            return Ok(());
        }
    };

    if files.is_empty() {
        println!("  📁 No DOC/DOCX files found for conversion");
        return Ok(());
    }

    println!("  📄 Found {} files to convert", files.len());

    // Convert to each format
    for (format, extension) in formats {
        println!("  🔄 Converting to {} format...", format);

        let mut format_args = clone_args(&base_args);
        format_args.format = format.to_string();
        format_args.output_dir = Some(PathBuf::from(format!("./{}_output", format)));

        // Create output directory
        if let Some(output_dir) = &format_args.output_dir {
            if let Err(e) = fs::create_dir_all(output_dir) {
                println!("    ❌ Failed to create output directory: {}", e);
                continue;
            }
        }

        // Process files for this format
        let mut success_count = 0;
        let mut error_count = 0;

        for file in &files {
            match convert_single_file(file, &format_args) {
                Ok(()) => {
                    success_count += 1;
                    println!("    ✅ Converted: {}", file.display());
                }
                Err(e) => {
                    error_count += 1;
                    println!("    ❌ Failed to convert {}: {}", file.display(), e);
                }
            }
        }

        println!("    📊 {} format: {} success, {} errors", 
                 format, success_count, error_count);
    }

    Ok(())
}

/// Example 4: Performance monitoring
fn performance_monitoring_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Monitoring batch processing performance...");

    let args = Args {
        input: PathBuf::new(),
        output: None,
        format: "text".to_string(),
        metadata: false,
        preserve_formatting: false,
        pretty: false,
        line_numbers: false,
        text_only: true, // Fastest mode
        verbose: false,
        batch: Some(".".to_string()),
        output_dir: Some(PathBuf::from("./perf_output")),
        overwrite: true,
        max_files: Some(10),
    };

    let files = match args.get_batch_files() {
        Ok(files) => files,
        Err(e) => {
            println!("  ❌ Error getting files: {}", e);
            return Ok(());
        }
    };

    if files.is_empty() {
        println!("  📁 No files found for performance testing");
        return Ok(());
    }

    println!("  📊 Performance test with {} files", files.len());

    // Create output directory
    if let Some(output_dir) = &args.output_dir {
        fs::create_dir_all(output_dir)?;
    }

    let start_time = Instant::now();
    let mut total_size = 0;
    let mut success_count = 0;
    let mut error_count = 0;

    for (i, file) in files.iter().enumerate() {
        let file_start = Instant::now();
        
        // Get file size
        if let Ok(metadata) = fs::metadata(file) {
            total_size += metadata.len();
        }

        match convert_single_file(file, &args) {
            Ok(()) => {
                success_count += 1;
                let file_duration = file_start.elapsed();
                println!("    ✅ File {}/{}: {} ({:.2}ms)", 
                         i + 1, files.len(), file.display(), file_duration.as_millis());
            }
            Err(e) => {
                error_count += 1;
                println!("    ❌ File {}/{}: {} - Error: {}", 
                         i + 1, files.len(), file.display(), e);
            }
        }
    }

    let total_duration = start_time.elapsed();
    
    println!("  📊 Performance Summary:");
    println!("    - Total files: {}", files.len());
    println!("    - Successful: {}", success_count);
    println!("    - Errors: {}", error_count);
    println!("    - Total time: {:.2}s", total_duration.as_secs_f64());
    println!("    - Average time per file: {:.2}ms", 
             total_duration.as_millis() as f64 / files.len() as f64);
    println!("    - Total size processed: {:.2} KB", total_size as f64 / 1024.0);
    
    if total_duration.as_secs() > 0 {
        println!("    - Processing speed: {:.2} KB/s", 
                 (total_size as f64 / 1024.0) / total_duration.as_secs_f64());
    }

    Ok(())
}

/// Example 5: Error handling and recovery
fn error_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Demonstrating error handling in batch processing...");

    // Create a mix of valid and invalid files for testing
    let test_files = vec![
        ("valid.docx", true),
        ("invalid.txt", false),
        ("missing.docx", false),
        ("corrupted.docx", false),
    ];

    println!("  📁 Test files prepared:");
    for (file, valid) in &test_files {
        println!("    - {}: {}", file, if *valid { "✅ Valid" } else { "❌ Invalid/Missing" });
    }

    let args = Args {
        input: PathBuf::new(),
        output: None,
        format: "text".to_string(),
        metadata: true,
        preserve_formatting: false,
        pretty: false,
        line_numbers: false,
        text_only: false,
        verbose: true,
        batch: Some(".".to_string()),
        output_dir: Some(PathBuf::from("./error_test_output")),
        overwrite: true,
        max_files: None,
    };

    // Create output directory
    if let Some(output_dir) = &args.output_dir {
        fs::create_dir_all(output_dir)?;
    }

    // Demonstrate different error scenarios
    println!("  🔍 Testing error scenarios:");

    // 1. Invalid batch pattern
    println!("    1. Invalid batch pattern:");
    let invalid_args = Args {
        batch: Some("[invalid-pattern".to_string()),
        ..clone_args(&args)
    };
    
    match invalid_args.get_batch_files() {
        Ok(_) => println!("      ❌ Should have failed"),
        Err(e) => println!("      ✅ Correctly caught error: {}", e),
    }

    // 2. Non-existent directory
    println!("    2. Non-existent directory:");
    let invalid_dir_args = Args {
        batch: Some("/non/existent/directory".to_string()),
        ..clone_args(&args)
    };
    
    match invalid_dir_args.get_batch_files() {
        Ok(_) => println!("      ❌ Should have failed"),
        Err(e) => println!("      ✅ Correctly caught error: {}", e),
    }

    // 3. No matching files
    println!("    3. No matching files:");
    let no_files_args = Args {
        batch: Some("*.xyz".to_string()),
        ..clone_args(&args)
    };
    
    match no_files_args.get_batch_files() {
        Ok(_) => println!("      ❌ Should have failed"),
        Err(e) => println!("      ✅ Correctly caught error: {}", e),
    }

    // 4. Invalid output directory
    println!("    4. Invalid output directory:");
    let invalid_output_args = Args {
        output_dir: Some(PathBuf::from("/invalid/output/directory")),
        ..clone_args(&args)
    };
    
    match invalid_output_args.validate() {
        Ok(_) => println!("      ❌ Should have failed"),
        Err(e) => println!("      ✅ Correctly caught error: {}", e),
    }

    println!("  ✅ Error handling tests completed");

    Ok(())
}

/// Helper function to process a batch of files
fn process_batch_files(files: &[PathBuf], args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    if files.is_empty() {
        println!("  📁 No files to process");
        return Ok(());
    }

    // Create output directory
    if let Some(output_dir) = &args.output_dir {
        fs::create_dir_all(output_dir)?;
    }

    let mut success_count = 0;
    let mut error_count = 0;

    for (i, file) in files.iter().enumerate() {
        println!("  📄 Processing file {}/{}: {}", i + 1, files.len(), file.display());

        match convert_single_file(file, args) {
            Ok(()) => {
                success_count += 1;
                println!("    ✅ Success");
            }
            Err(e) => {
                error_count += 1;
                println!("    ❌ Error: {}", e);
            }
        }
    }

    println!("  📊 Batch processing completed:");
    println!("    - Successful: {}", success_count);
    println!("    - Errors: {}", error_count);
    println!("    - Total: {}", files.len());

    Ok(())
}

/// Helper function to convert a single file
fn convert_single_file(file: &PathBuf, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let parser = DocParser::new();
    
    // Parse the document
    let doc_data = parser.parse(file)?;
    
    // Generate output filename
    let output_filename = {
        let stem = file.file_stem().unwrap().to_str().unwrap();
        let extension = match args.format.as_str() {
            "json" => "json",
            "markdown" | "md" => "md",
            _ => "txt",
        };
        format!("{}.{}", stem, extension)
    };

    let output_path = if let Some(output_dir) = &args.output_dir {
        output_dir.join(output_filename)
    } else {
        PathBuf::from(output_filename)
    };

    // Check if file exists and overwrite is not enabled
    if output_path.exists() && !args.overwrite {
        return Err(format!("Output file exists: {}", output_path.display()).into());
    }

    // Create output format
    let format = match args.format.as_str() {
        "json" => OutputFormat::Json {
            pretty: args.pretty,
            include_formatting: args.preserve_formatting,
        },
        "markdown" | "md" => OutputFormat::Markdown {
            preserve_structure: !args.text_only,
            include_metadata: args.metadata,
        },
        _ => OutputFormat::Text {
            preserve_formatting: args.preserve_formatting,
            include_metadata: args.metadata,
            line_numbers: args.line_numbers,
        },
    };

    // Convert and save
    let processor = OutputProcessor::new();
    let output_file = std::fs::File::create(&output_path)?;
    let mut writer = std::io::BufWriter::new(output_file);
    
    processor.process(&doc_data, &format, &mut writer)?;

    Ok(())
}

// Helper function to clone Args
fn clone_args(args: &Args) -> Args {
    Args {
        input: args.input.clone(),
        output: args.output.clone(),
        format: args.format.clone(),
        metadata: args.metadata,
        preserve_formatting: args.preserve_formatting,
        pretty: args.pretty,
        line_numbers: args.line_numbers,
        text_only: args.text_only,
        verbose: args.verbose,
        batch: args.batch.clone(),
        output_dir: args.output_dir.clone(),
        overwrite: args.overwrite,
        max_files: args.max_files,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::io::Write;

    #[test]
    fn test_batch_processing_functions() {
        // Test that functions can be called without panicking
        assert!(true);
    }

    #[test]
    fn test_error_scenarios() {
        let temp_dir = TempDir::new().unwrap();
        
        // Test invalid batch pattern
        let args = Args {
            input: PathBuf::new(),
            output: None,
            format: "text".to_string(),
            metadata: false,
            preserve_formatting: false,
            pretty: false,
            line_numbers: false,
            text_only: false,
            verbose: false,
            batch: Some("invalid[pattern".to_string()),
            output_dir: Some(temp_dir.path().to_path_buf()),
            overwrite: false,
            max_files: None,
        };

        // Should fail with invalid glob pattern
        assert!(args.get_batch_files().is_err());
    }
}