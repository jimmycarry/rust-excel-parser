use doc_parser::{Args, DocParser, OutputProcessor, Result};
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

fn main() -> Result<()> {
    let args = Args::parse_args();

    // 验证参数
    if let Err(e) = args.validate() {
        eprintln!("{}", e.user_friendly_message());
        std::process::exit(1);
    }

    // 检查是否为批处理模式
    if args.is_batch_mode() {
        return process_batch(&args);
    }

    // 单文件处理模式
    process_single_file(&args)
}

fn process_batch(args: &Args) -> Result<()> {
    let files = args.get_batch_files()?;
    
    if args.verbose {
        eprintln!("📂 Found {} files to process", files.len());
    }

    let mut success_count = 0;
    let mut error_count = 0;

    for (index, file_path) in files.iter().enumerate() {
        if args.verbose {
            eprintln!("📄 Processing file {}/{}: {}", index + 1, files.len(), file_path.display());
        }

        // 生成输出文件名
        let output_path = generate_output_path(file_path, &args.format, &args.output_dir)?;
        
        // 检查文件是否存在，如果存在且不覆盖则跳过
        if output_path.exists() && !args.overwrite {
            if args.verbose {
                eprintln!("⚠️  Output file exists, skipping: {}", output_path.display());
            }
            continue;
        }

        // 处理单个文件
        match process_single_file_internal(file_path, Some(&output_path), args) {
            Ok(()) => {
                success_count += 1;
                if args.verbose {
                    eprintln!("✅ Successfully processed: {}", file_path.display());
                }
            }
            Err(e) => {
                error_count += 1;
                eprintln!("❌ Error processing {}: {}", file_path.display(), e.user_friendly_message());
            }
        }
    }

    // 打印总结
    if args.verbose || error_count > 0 {
        eprintln!("\n📊 Batch processing completed:");
        eprintln!("   ✅ Successfully processed: {}", success_count);
        if error_count > 0 {
            eprintln!("   ❌ Errors: {}", error_count);
        }
    }

    Ok(())
}

fn generate_output_path(input_path: &Path, format: &str, output_dir: &Option<std::path::PathBuf>) -> Result<std::path::PathBuf> {
    let input_stem = input_path.file_stem()
        .ok_or_else(|| doc_parser::error::DocParserError::InvalidStructure {
            file: input_path.display().to_string(),
            details: "Cannot extract filename".to_string(),
        })?
        .to_str()
        .ok_or_else(|| doc_parser::error::DocParserError::Encoding {
            file: input_path.display().to_string(),
            details: "Invalid UTF-8 in filename".to_string(),
        })?;

    let extension = match format.to_lowercase().as_str() {
        "text" => "txt",
        "markdown" | "md" => "md",
        "json" => "json",
        _ => "txt",
    };

    let output_filename = format!("{}.{}", input_stem, extension);
    
    let output_path = if let Some(dir) = output_dir {
        dir.join(output_filename)
    } else {
        std::env::current_dir()?
            .join(output_filename)
    };

    Ok(output_path)
}

fn process_single_file(args: &Args) -> Result<()> {
    process_single_file_internal(&args.input, args.output.as_deref(), args)
}

fn process_single_file_internal(input_path: &Path, output_path: Option<&Path>, args: &Args) -> Result<()> {
    if args.verbose {
        eprintln!("Parsing file: {}", input_path.display());
        eprintln!("Processing mode: {:?}", args.get_processing_mode());
    }

    // 创建解析器
    let parser = DocParser::new();

    // 根据处理模式解析文档
    let doc_data = match args.get_processing_mode() {
        doc_parser::cli::ProcessingMode::TextOnly => {
            // 只提取文本，最快模式
            if args.verbose {
                eprintln!("Extracting text only (fast mode)");
            }
            
            let text = parser.extract_text(input_path)?;
            
            // 直接输出文本，不经过复杂的格式处理
            match output_path {
                Some(output_path) => {
                    if args.verbose {
                        eprintln!("Writing to file: {}", output_path.display());
                    }
                    let file = File::create(output_path)?;
                    let mut writer = BufWriter::new(file);
                    writeln!(writer, "{}", text)?;
                    writer.flush()?;
                }
                None => {
                    if args.verbose {
                        eprintln!("Writing to stdout");
                    }
                    println!("{}", text);
                }
            }
            
            if args.verbose {
                let word_count = text.split_whitespace().count();
                eprintln!("Extraction completed successfully - {} words", word_count);
            }
            
            return Ok(());
        }
        _ => {
            // 完整解析
            if args.verbose {
                eprintln!("Parsing document structure");
            }
            parser.parse(input_path)?
        }
    };

    if args.verbose {
        eprintln!("Document parsed successfully:");
        eprintln!("  - Paragraphs: {}", doc_data.metadata.paragraph_count);
        eprintln!("  - Words: {}", doc_data.metadata.word_count);
        eprintln!("  - Characters: {}", doc_data.metadata.character_count);
        eprintln!("  - Sections: {}", doc_data.sections.len());
        
        if let Some(title) = &doc_data.metadata.title {
            eprintln!("  - Title: {}", title);
        }
    }

    // 获取输出格式
    let format = args.get_output_format()?;

    // 创建输出处理器
    let processor = OutputProcessor::new();

    // 写入输出
    match output_path {
        Some(output_path) => {
            if args.verbose {
                eprintln!("Writing to file: {}", output_path.display());
            }
            let file = File::create(output_path)?;
            let mut writer = BufWriter::new(file);
            processor.process(&doc_data, &format, &mut writer)?;
            writer.flush()?;
        }
        None => {
            if args.verbose {
                eprintln!("Writing to stdout");
            }
            let stdout = io::stdout();
            let mut writer = stdout.lock();
            processor.process(&doc_data, &format, &mut writer)?;
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
    use super::*;

    #[test]
    fn test_generate_output_path() {
        let input_path = Path::new("test.docx");
        let output_path = generate_output_path(input_path, "json", &None).unwrap();
        assert!(output_path.to_string_lossy().ends_with("test.json"));

        let output_path = generate_output_path(input_path, "markdown", &None).unwrap();
        assert!(output_path.to_string_lossy().ends_with("test.md"));

        let output_path = generate_output_path(input_path, "text", &None).unwrap();
        assert!(output_path.to_string_lossy().ends_with("test.txt"));
    }
}