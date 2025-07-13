use crate::error::Result;
use crate::output::OutputWriter;
use crate::parser::{PptData, Slide, ListType};
use std::io::Write;

pub struct HtmlOutput {
    include_metadata: bool,
    include_css: bool,
}

impl HtmlOutput {
    pub fn new(include_metadata: bool, include_css: bool) -> Self {
        Self {
            include_metadata,
            include_css,
        }
    }

    fn write_css<W: Write>(&self, writer: &mut W) -> Result<()> {
        writeln!(writer, r#"<style>
body {{
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    line-height: 1.6;
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    background-color: #f5f5f5;
}}

.presentation {{
    background-color: white;
    padding: 30px;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}}

.presentation-title {{
    color: #2c3e50;
    border-bottom: 3px solid #3498db;
    padding-bottom: 10px;
    margin-bottom: 30px;
}}

.metadata {{
    background-color: #ecf0f1;
    padding: 20px;
    border-radius: 5px;
    margin-bottom: 30px;
}}

.metadata h2 {{
    color: #34495e;
    margin-top: 0;
}}

.metadata-item {{
    margin: 5px 0;
}}

.slide {{
    background-color: #fff;
    border: 1px solid #bdc3c7;
    border-radius: 5px;
    padding: 25px;
    margin: 20px 0;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}}

.slide-header {{
    color: #2980b9;
    border-bottom: 2px solid #3498db;
    padding-bottom: 10px;
    margin-bottom: 20px;
}}

.slide-content {{
    margin: 15px 0;
}}

.slide-content p {{
    margin: 10px 0;
    color: #2c3e50;
}}

.slide-notes {{
    background-color: #fff3cd;
    border: 1px solid #ffeaa7;
    border-radius: 4px;
    padding: 15px;
    margin-top: 20px;
}}

.slide-notes h4 {{
    margin: 0 0 10px 0;
    color: #856404;
}}

table {{
    width: 100%;
    border-collapse: collapse;
    margin: 15px 0;
    background-color: white;
}}

table th, table td {{
    border: 1px solid #bdc3c7;
    padding: 12px;
    text-align: left;
}}

table th {{
    background-color: #3498db;
    color: white;
    font-weight: bold;
}}

table tr:nth-child(even) {{
    background-color: #f8f9fa;
}}

ul, ol {{
    margin: 15px 0;
    padding-left: 30px;
}}

li {{
    margin: 5px 0;
    color: #2c3e50;
}}

.slide-number {{
    background-color: #3498db;
    color: white;
    padding: 5px 10px;
    border-radius: 15px;
    font-size: 12px;
    font-weight: bold;
    display: inline-block;
    margin-bottom: 10px;
}}
</style>"#)?;
        Ok(())
    }

    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }
}

impl OutputWriter for HtmlOutput {
    fn write_ppt_data<W: Write>(&self, data: &PptData, writer: &mut W) -> Result<()> {
        // Write HTML header
        writeln!(writer, "<!DOCTYPE html>")?;
        writeln!(writer, "<html lang=\"en\">")?;
        writeln!(writer, "<head>")?;
        writeln!(writer, "    <meta charset=\"UTF-8\">")?;
        writeln!(writer, "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">")?;
        
        if let Some(title) = &data.metadata.title {
            writeln!(writer, "    <title>{}</title>", self.escape_html(title))?;
        } else {
            writeln!(writer, "    <title>Presentation</title>")?;
        }

        if self.include_css {
            self.write_css(writer)?;
        }

        writeln!(writer, "</head>")?;
        writeln!(writer, "<body>")?;
        writeln!(writer, "<div class=\"presentation\">")?;

        // Write presentation title
        if let Some(title) = &data.metadata.title {
            writeln!(writer, "    <h1 class=\"presentation-title\">{}</h1>", self.escape_html(title))?;
        }

        // Write metadata if requested
        if self.include_metadata {
            writeln!(writer, "    <div class=\"metadata\">")?;
            writeln!(writer, "        <h2>Presentation Information</h2>")?;
            
            if let Some(author) = &data.metadata.author {
                writeln!(writer, "        <div class=\"metadata-item\"><strong>Author:</strong> {}</div>", self.escape_html(author))?;
            }
            
            if let Some(subject) = &data.metadata.subject {
                writeln!(writer, "        <div class=\"metadata-item\"><strong>Subject:</strong> {}</div>", self.escape_html(subject))?;
            }
            
            if let Some(creation_date) = &data.metadata.creation_date {
                writeln!(writer, "        <div class=\"metadata-item\"><strong>Created:</strong> {}</div>", creation_date.format("%Y-%m-%d %H:%M:%S UTC"))?;
            }
            
            writeln!(writer, "        <div class=\"metadata-item\"><strong>Slides:</strong> {}</div>", data.slide_count)?;
            writeln!(writer, "        <div class=\"metadata-item\"><strong>File Size:</strong> {} bytes</div>", data.metadata.file_size)?;
            writeln!(writer, "    </div>")?;
        }

        // Write each slide
        for slide in &data.slides {
            writeln!(writer, "    <div class=\"slide\">")?;
            self.write_slide_content(slide, writer)?;
            writeln!(writer, "    </div>")?;
        }

        writeln!(writer, "</div>")?;
        writeln!(writer, "</body>")?;
        writeln!(writer, "</html>")?;

        Ok(())
    }

    fn write_slide<W: Write>(&self, slide: &Slide, writer: &mut W) -> Result<()> {
        // Write minimal HTML for single slide
        writeln!(writer, "<!DOCTYPE html>")?;
        writeln!(writer, "<html lang=\"en\">")?;
        writeln!(writer, "<head>")?;
        writeln!(writer, "    <meta charset=\"UTF-8\">")?;
        writeln!(writer, "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">")?;
        
        if let Some(title) = &slide.title {
            writeln!(writer, "    <title>Slide {}: {}</title>", slide.number, self.escape_html(title))?;
        } else {
            writeln!(writer, "    <title>Slide {}</title>", slide.number)?;
        }

        if self.include_css {
            self.write_css(writer)?;
        }

        writeln!(writer, "</head>")?;
        writeln!(writer, "<body>")?;
        writeln!(writer, "<div class=\"presentation\">")?;
        writeln!(writer, "    <div class=\"slide\">")?;
        
        self.write_slide_content(slide, writer)?;
        
        writeln!(writer, "    </div>")?;
        writeln!(writer, "</div>")?;
        writeln!(writer, "</body>")?;
        writeln!(writer, "</html>")?;

        Ok(())
    }
}

impl HtmlOutput {
    fn write_slide_content<W: Write>(&self, slide: &Slide, writer: &mut W) -> Result<()> {
        // Write slide header
        writeln!(writer, "        <div class=\"slide-number\">Slide {}</div>", slide.number)?;
        
        if let Some(title) = &slide.title {
            writeln!(writer, "        <h2 class=\"slide-header\">{}</h2>", self.escape_html(title))?;
        }

        // Write content
        if !slide.content.is_empty() {
            writeln!(writer, "        <div class=\"slide-content\">")?;
            for content_item in &slide.content {
                writeln!(writer, "            <p>{}</p>", self.escape_html(content_item))?;
            }
            writeln!(writer, "        </div>")?;
        }

        // Write lists
        for list in &slide.lists {
            match list.list_type {
                ListType::Ordered => {
                    writeln!(writer, "        <ol>")?;
                    for item in &list.items {
                        writeln!(writer, "            <li>{}</li>", self.escape_html(item))?;
                    }
                    writeln!(writer, "        </ol>")?;
                }
                ListType::Unordered => {
                    writeln!(writer, "        <ul>")?;
                    for item in &list.items {
                        writeln!(writer, "            <li>{}</li>", self.escape_html(item))?;
                    }
                    writeln!(writer, "        </ul>")?;
                }
            }
        }

        // Write tables
        for table in &slide.tables {
            if !table.rows.is_empty() {
                writeln!(writer, "        <table>")?;
                
                // Write headers
                if let Some(headers) = &table.headers {
                    writeln!(writer, "            <thead>")?;
                    writeln!(writer, "                <tr>")?;
                    for header in headers {
                        writeln!(writer, "                    <th>{}</th>", self.escape_html(header))?;
                    }
                    writeln!(writer, "                </tr>")?;
                    writeln!(writer, "            </thead>")?;
                    
                    // Write data rows (skip first row if it's headers)
                    writeln!(writer, "            <tbody>")?;
                    for row in &table.rows[1..] {
                        writeln!(writer, "                <tr>")?;
                        for cell in row {
                            writeln!(writer, "                    <td>{}</td>", self.escape_html(cell))?;
                        }
                        writeln!(writer, "                </tr>")?;
                    }
                    writeln!(writer, "            </tbody>")?;
                } else {
                    // Treat first row as headers
                    if !table.rows.is_empty() {
                        writeln!(writer, "            <thead>")?;
                        writeln!(writer, "                <tr>")?;
                        for header in &table.rows[0] {
                            writeln!(writer, "                    <th>{}</th>", self.escape_html(header))?;
                        }
                        writeln!(writer, "                </tr>")?;
                        writeln!(writer, "            </thead>")?;
                        
                        writeln!(writer, "            <tbody>")?;
                        for row in &table.rows[1..] {
                            writeln!(writer, "                <tr>")?;
                            for cell in row {
                                writeln!(writer, "                    <td>{}</td>", self.escape_html(cell))?;
                            }
                            writeln!(writer, "                </tr>")?;
                        }
                        writeln!(writer, "            </tbody>")?;
                    }
                }
                
                writeln!(writer, "        </table>")?;
            }
        }

        // Write notes
        if let Some(notes) = &slide.notes {
            writeln!(writer, "        <div class=\"slide-notes\">")?;
            writeln!(writer, "            <h4>Speaker Notes</h4>")?;
            writeln!(writer, "            <p>{}</p>", self.escape_html(notes))?;
            writeln!(writer, "        </div>")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{List, ListType, PptMetadata, Table};
    use chrono::Utc;

    #[test]
    fn test_html_escape() {
        let output = HtmlOutput::new(false, false);
        assert_eq!(output.escape_html("Test & <script>"), "Test &amp; &lt;script&gt;");
        assert_eq!(output.escape_html("Quote \"test\""), "Quote &quot;test&quot;");
    }

    #[test]
    fn test_html_slide_output() {
        let slide = Slide {
            number: 1,
            title: Some("Test Slide".to_string()),
            content: vec!["First point".to_string(), "Second point".to_string()],
            notes: Some("Important notes".to_string()),
            tables: vec![],
            lists: vec![],
        };

        let output = HtmlOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("<!DOCTYPE html>"));
        assert!(result.contains("<title>Slide 1: Test Slide</title>"));
        assert!(result.contains("<h2 class=\"slide-header\">Test Slide</h2>"));
        assert!(result.contains("<p>First point</p>"));
        assert!(result.contains("<p>Second point</p>"));
        assert!(result.contains("Speaker Notes"));
        assert!(result.contains("<p>Important notes</p>"));
    }

    #[test]
    fn test_html_slide_with_list() {
        let list = List {
            slide_number: 1,
            list_type: ListType::Unordered,
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
        };

        let slide = Slide {
            number: 1,
            title: None,
            content: vec![],
            notes: None,
            tables: vec![],
            lists: vec![list],
        };

        let output = HtmlOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("<ul>"));
        assert!(result.contains("<li>Item 1</li>"));
        assert!(result.contains("<li>Item 2</li>"));
        assert!(result.contains("</ul>"));
    }

    #[test]
    fn test_html_slide_with_ordered_list() {
        let list = List {
            slide_number: 1,
            list_type: ListType::Ordered,
            items: vec!["First".to_string(), "Second".to_string()],
        };

        let slide = Slide {
            number: 1,
            title: None,
            content: vec![],
            notes: None,
            tables: vec![],
            lists: vec![list],
        };

        let output = HtmlOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("<ol>"));
        assert!(result.contains("<li>First</li>"));
        assert!(result.contains("<li>Second</li>"));
        assert!(result.contains("</ol>"));
    }

    #[test]
    fn test_html_slide_with_table() {
        let table = Table {
            slide_number: 1,
            rows: vec![
                vec!["Name".to_string(), "Age".to_string()],
                vec!["John".to_string(), "30".to_string()],
                vec!["Jane".to_string(), "25".to_string()],
            ],
            headers: Some(vec!["Name".to_string(), "Age".to_string()]),
        };

        let slide = Slide {
            number: 1,
            title: None,
            content: vec![],
            notes: None,
            tables: vec![table],
            lists: vec![],
        };

        let output = HtmlOutput::new(false, false);
        let mut buffer = Vec::new();
        output.write_slide(&slide, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("<table>"));
        assert!(result.contains("<thead>"));
        assert!(result.contains("<th>Name</th>"));
        assert!(result.contains("<th>Age</th>"));
        assert!(result.contains("<tbody>"));
        assert!(result.contains("<td>John</td>"));
        assert!(result.contains("<td>30</td>"));
    }

    #[test]
    fn test_html_ppt_data_with_metadata() {
        let metadata = PptMetadata {
            title: Some("Test Presentation".to_string()),
            author: Some("Test Author".to_string()),
            subject: Some("Test Subject".to_string()),
            creator: None,
            creation_date: Some(Utc::now()),
            modification_date: None,
            slide_count: 1,
            file_size: 1024,
            application: None,
        };

        let slide = Slide {
            number: 1,
            title: Some("Slide 1".to_string()),
            content: vec!["Content".to_string()],
            notes: None,
            tables: vec![],
            lists: vec![],
        };

        let ppt_data = PptData {
            slides: vec![slide],
            metadata,
            slide_count: 1,
        };

        let output = HtmlOutput::new(true, true);
        let mut buffer = Vec::new();
        output.write_ppt_data(&ppt_data, &mut buffer).unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("<title>Test Presentation</title>"));
        assert!(result.contains("<h1 class=\"presentation-title\">Test Presentation</h1>"));
        assert!(result.contains("<div class=\"metadata\">"));
        assert!(result.contains("<strong>Author:</strong> Test Author"));
        assert!(result.contains("<strong>Subject:</strong> Test Subject"));
        assert!(result.contains("<strong>Slides:</strong> 1"));
        assert!(result.contains("<style>"));
        assert!(result.contains("font-family:"));
    }
}