# Troubleshooting Guide

This guide helps you diagnose and resolve common issues with the doc-parser library.

## Table of Contents

1. [Common Error Messages](#common-error-messages)
2. [File-Related Issues](#file-related-issues)
3. [Parsing Issues](#parsing-issues)
4. [Performance Issues](#performance-issues)
5. [CLI Issues](#cli-issues)
6. [Build and Installation Issues](#build-and-installation-issues)
7. [Platform-Specific Issues](#platform-specific-issues)
8. [Debug Mode](#debug-mode)
9. [Getting Help](#getting-help)

## Common Error Messages

### "File not found"

**Error Message:**
```
❌ File not found: 'document.docx'
💡 Make sure the file path is correct and the file exists.
```

**Cause:** The specified file doesn't exist at the given path.

**Solutions:**
1. **Check the file path:**
   ```bash
   ls -la document.docx  # Linux/macOS
   dir document.docx     # Windows
   ```

2. **Use absolute paths:**
   ```bash
   doc-parser /full/path/to/document.docx
   ```

3. **Check current directory:**
   ```bash
   pwd  # Shows current directory
   ```

4. **Verify file permissions:**
   ```bash
   ls -la document.docx  # Check if file is readable
   ```

### "Unsupported file format"

**Error Message:**
```
❌ Unsupported file format: '.txt'
💡 Supported formats: .doc, .docx
💡 Try converting your file to Word format first.
```

**Cause:** The file is not a DOC or DOCX file.

**Solutions:**
1. **Check file extension:**
   ```bash
   file document.docx  # Shows actual file type
   ```

2. **Convert file to supported format:**
   - Use Microsoft Word to save as DOCX
   - Use LibreOffice to convert to DOCX
   - Use online converters

3. **Rename file if extension is wrong:**
   ```bash
   mv document.doc document.docx  # If it's actually a DOCX file
   ```

### "Failed to parse DOCX"

**Error Message:**
```
❌ Failed to parse DOCX file: 'document.docx'
💡 The file might be corrupted or not a valid DOCX file.
```

**Cause:** The file is corrupted or not a valid DOCX format.

**Solutions:**
1. **Try opening in Microsoft Word:**
   - If Word can't open it, the file is corrupted
   - Word might be able to repair it

2. **Check file integrity:**
   ```bash
   unzip -t document.docx  # DOCX files are ZIP archives
   ```

3. **Re-download or re-create the file:**
   - Download again if from internet
   - Re-export from original source

4. **Use repair tools:**
   - Online DOCX repair tools
   - Microsoft Word's built-in repair

### "Permission denied"

**Error Message:**
```
❌ Permission denied: Cannot read file 'document.docx'
💡 Check file permissions and try again.
```

**Cause:** Insufficient permissions to read the file.

**Solutions:**
1. **Check file permissions:**
   ```bash
   ls -la document.docx
   ```

2. **Fix permissions:**
   ```bash
   chmod 644 document.docx  # Linux/macOS
   ```

3. **Run as administrator (Windows):**
   ```bash
   # Run command prompt as administrator
   ```

4. **Change file ownership:**
   ```bash
   sudo chown $USER document.docx  # Linux/macOS
   ```

### "Output directory does not exist"

**Error Message:**
```
❌ Output directory doesn't exist: './output'
💡 Create the directory first: mkdir -p './output'
```

**Cause:** The specified output directory doesn't exist.

**Solutions:**
1. **Create directory:**
   ```bash
   mkdir -p ./output
   ```

2. **Use existing directory:**
   ```bash
   doc-parser document.docx --output-dir ./existing-dir
   ```

3. **Use current directory:**
   ```bash
   doc-parser document.docx --output-dir .
   ```

## File-Related Issues

### Large Files

**Problem:** Processing very large DOCX files (>100MB) is slow or fails.

**Solutions:**
1. **Use text-only mode:**
   ```bash
   doc-parser large-document.docx --text-only
   ```

2. **Increase memory limits:**
   ```bash
   export RUST_MIN_STACK=8388608  # 8MB stack
   ```

3. **Split document into smaller parts:**
   - Use Word to split into multiple documents
   - Process parts separately

### Password-Protected Files

**Problem:** Cannot parse password-protected DOCX files.

**Current Status:** Password-protected files are not supported.

**Workarounds:**
1. **Remove password in Word:**
   - Open in Microsoft Word
   - File → Info → Protect Document → Remove Password

2. **Save as unprotected copy:**
   - Save as new file without password protection

### Corrupted Files

**Problem:** Files that appear corrupted or produce unexpected output.

**Diagnostic Steps:**
1. **Test with minimal document:**
   ```bash
   # Create a simple test document
   doc-parser test-simple.docx -v
   ```

2. **Check ZIP structure:**
   ```bash
   unzip -l document.docx | head -20
   ```

3. **Extract and examine:**
   ```bash
   mkdir temp-docx
   cd temp-docx
   unzip ../document.docx
   ls -la
   ```

## Parsing Issues

### Missing Content

**Problem:** Some content is missing from the parsed output.

**Causes and Solutions:**

1. **Complex formatting:**
   - Try with `--preserve-formatting` flag
   - Use different output format (JSON for full structure)

2. **Embedded objects:**
   - Images and objects are not fully supported
   - Use `--verbose` to see what's being processed

3. **Headers and footers:**
   - Limited support for headers/footers
   - Content might be in separate sections

### Incorrect Structure

**Problem:** Document structure is not preserved correctly.

**Solutions:**
1. **Use structured output:**
   ```bash
   doc-parser document.docx -f json --pretty
   ```

2. **Try markdown output:**
   ```bash
   doc-parser document.docx -f markdown --metadata
   ```

3. **Enable verbose mode:**
   ```bash
   doc-parser document.docx -v
   ```

### Encoding Issues

**Problem:** Special characters or non-English text appears garbled.

**Solutions:**
1. **Check system locale:**
   ```bash
   locale  # Linux/macOS
   ```

2. **Set UTF-8 encoding:**
   ```bash
   export LANG=en_US.UTF-8
   export LC_ALL=en_US.UTF-8
   ```

3. **Use JSON output:**
   ```bash
   doc-parser document.docx -f json --pretty
   ```

## Performance Issues

### Slow Processing

**Problem:** Document processing is very slow.

**Solutions:**
1. **Use text-only mode:**
   ```bash
   doc-parser document.docx --text-only
   ```

2. **Disable metadata extraction:**
   ```bash
   doc-parser document.docx  # Don't use --metadata
   ```

3. **Check system resources:**
   ```bash
   top  # Check CPU and memory usage
   ```

4. **Use batch processing efficiently:**
   ```bash
   doc-parser --batch ./docs --max-files 10
   ```

### Memory Issues

**Problem:** High memory usage or out-of-memory errors.

**Solutions:**
1. **Increase available memory:**
   ```bash
   ulimit -v 2097152  # 2GB virtual memory limit
   ```

2. **Process smaller batches:**
   ```bash
   doc-parser --batch ./docs --max-files 5
   ```

3. **Use text-only mode:**
   ```bash
   doc-parser --batch ./docs --text-only
   ```

## CLI Issues

### Invalid Arguments

**Problem:** Command-line arguments are not working as expected.

**Solutions:**
1. **Check help text:**
   ```bash
   doc-parser --help
   ```

2. **Verify argument format:**
   ```bash
   # Correct format
   doc-parser document.docx -f json --pretty
   
   # Common mistakes
   doc-parser document.docx --format=json  # Don't use =
   doc-parser -f json document.docx       # Put filename first
   ```

3. **Check for typos:**
   ```bash
   doc-parser document.docx --metadta  # Should be --metadata
   ```

### Batch Processing Issues

**Problem:** Batch processing not finding files or failing.

**Solutions:**
1. **Test glob patterns:**
   ```bash
   ls *.docx  # Test pattern manually
   ```

2. **Use quotes for complex patterns:**
   ```bash
   doc-parser --batch "**/*.docx"
   ```

3. **Check directory permissions:**
   ```bash
   ls -la ./documents/
   ```

4. **Use absolute paths:**
   ```bash
   doc-parser --batch /full/path/to/documents
   ```

## Build and Installation Issues

### Compilation Errors

**Problem:** Build fails with compilation errors.

**Solutions:**
1. **Update Rust toolchain:**
   ```bash
   rustup update
   ```

2. **Check minimum version:**
   ```bash
   rustc --version  # Should be 1.70+
   ```

3. **Clean build cache:**
   ```bash
   cargo clean
   cargo build
   ```

4. **Update dependencies:**
   ```bash
   cargo update
   ```

### Missing Dependencies

**Problem:** Build fails due to missing system dependencies.

**Solutions:**

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev
```

**Linux (CentOS/RHEL):**
```bash
sudo yum groupinstall "Development Tools"
sudo yum install openssl-devel
```

**macOS:**
```bash
xcode-select --install  # Install Xcode command line tools
```

**Windows:**
- Install Visual Studio Build Tools
- Install Rust from https://rustup.rs/

### Feature Compilation Issues

**Problem:** Build fails when using specific features.

**Solutions:**
1. **Check feature availability:**
   ```bash
   cargo build --features legacy-doc
   ```

2. **Use default features:**
   ```bash
   cargo build  # Uses default features only
   ```

3. **Check feature dependencies:**
   ```bash
   cargo tree --features legacy-doc
   ```

## Platform-Specific Issues

### Windows

**Common Issues:**
1. **Path separators:**
   ```bash
   # Use forward slashes or double backslashes
   doc-parser "C:\\Documents\\file.docx"
   doc-parser "C:/Documents/file.docx"
   ```

2. **Long path names:**
   ```bash
   # Enable long paths in Windows or use shorter names
   ```

3. **Antivirus interference:**
   - Add exception for doc-parser executable
   - Temporarily disable real-time scanning

### macOS

**Common Issues:**
1. **Gatekeeper blocking:**
   ```bash
   sudo spctl --master-disable  # Disable Gatekeeper temporarily
   ```

2. **Permission issues:**
   ```bash
   # Grant full disk access in System Preferences
   ```

### Linux

**Common Issues:**
1. **Missing libraries:**
   ```bash
   ldd target/release/doc-parser  # Check library dependencies
   ```

2. **AppArmor/SELinux:**
   ```bash
   # Check security policies if file access is denied
   ```

## Debug Mode

### Enabling Debug Output

1. **Use verbose flag:**
   ```bash
   doc-parser document.docx -v
   ```

2. **Set environment variables:**
   ```bash
   export RUST_LOG=debug
   doc-parser document.docx
   ```

3. **Use debug build:**
   ```bash
   cargo build  # Debug build has more error info
   ./target/debug/doc-parser document.docx
   ```

### Debugging Parsing Issues

1. **Check document structure:**
   ```bash
   doc-parser document.docx -f json --pretty > debug.json
   ```

2. **Extract DOCX contents:**
   ```bash
   mkdir debug-docx
   unzip document.docx -d debug-docx
   ls -la debug-docx/
   ```

3. **Test with minimal document:**
   - Create simple test document
   - Compare with problematic document

### Log Analysis

Common log patterns to look for:

```
DEBUG: Parsing file: document.docx
DEBUG: Processing mode: Standard
DEBUG: Found 15 sections
DEBUG: Extraction completed successfully - 1234 words
```

Error patterns:
```
ERROR: Failed to parse DOCX: InvalidZipArchive
ERROR: Permission denied: /path/to/file
ERROR: Unsupported format: txt
```

## Getting Help

### Before Asking for Help

1. **Check this troubleshooting guide**
2. **Search existing issues on GitHub**
3. **Try with a simple test document**
4. **Collect debug information**

### Information to Include

When reporting issues, include:

1. **Version information:**
   ```bash
   doc-parser --version
   rustc --version
   ```

2. **Operating system:**
   ```bash
   uname -a    # Linux/macOS
   ver         # Windows
   ```

3. **Command used:**
   ```bash
   # Exact command that failed
   doc-parser document.docx -f json --pretty
   ```

4. **Error message:**
   ```bash
   # Complete error output
   ```

5. **File information:**
   ```bash
   file document.docx
   ls -la document.docx
   ```

6. **Minimal reproduction:**
   - Smallest possible document that shows the issue
   - Exact steps to reproduce

### Where to Get Help

1. **GitHub Issues:** [Repository Issues](https://github.com/your-repo/issues)
2. **Documentation:** README.md and this guide
3. **Examples:** Check the examples/ directory
4. **Community:** Rust community forums

### Creating Good Bug Reports

1. **Use descriptive titles:**
   - Good: "DOCX parsing fails with tables containing merged cells"
   - Bad: "Parser broken"

2. **Include minimal reproduction:**
   - Provide smallest possible failing case
   - Include exact commands and output

3. **Specify environment:**
   - OS version
   - Rust version
   - doc-parser version

4. **Describe expected vs actual behavior:**
   - What you expected to happen
   - What actually happened

### Performance Issues

For performance problems, include:

1. **File size and type:**
   ```bash
   ls -lh document.docx
   file document.docx
   ```

2. **Timing information:**
   ```bash
   time doc-parser document.docx
   ```

3. **Resource usage:**
   ```bash
   # Monitor during processing
   top -p $(pgrep doc-parser)
   ```

4. **Hardware specifications:**
   - CPU model
   - Available RAM
   - Storage type (SSD/HDD)

---

This troubleshooting guide covers the most common issues. If you encounter a problem not covered here, please create an issue on GitHub with detailed information about your problem.