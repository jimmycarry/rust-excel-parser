// use crate::error::Result;
use crate::parser::{DocData, DocSection};

/// 通用文本提取trait
pub trait TextExtractor {
    /// 提取纯文本
    fn extract_text(&self) -> String;
    
    /// 提取带格式信息的文本
    fn extract_formatted_text(&self) -> String;
    
    /// 获取字数统计
    fn word_count(&self) -> usize;
    
    /// 获取字符数统计
    fn character_count(&self) -> usize;
}

impl TextExtractor for DocData {
    fn extract_text(&self) -> String {
        self.raw_text.clone()
    }
    
    fn extract_formatted_text(&self) -> String {
        self.content.clone()
    }
    
    fn word_count(&self) -> usize {
        self.metadata.word_count
    }
    
    fn character_count(&self) -> usize {
        self.metadata.character_count
    }
}

impl TextExtractor for DocSection {
    fn extract_text(&self) -> String {
        self.content.clone()
    }
    
    fn extract_formatted_text(&self) -> String {
        match &self.section_type {
            crate::parser::SectionType::Heading(level) => {
                format!("{} {}", "#".repeat(*level as usize), self.content)
            }
            crate::parser::SectionType::List => {
                format!("- {}", self.content)
            }
            crate::parser::SectionType::Table => {
                format!("| {} |", self.content)
            }
            _ => self.content.clone(),
        }
    }
    
    fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
    
    fn character_count(&self) -> usize {
        self.content.chars().count()
    }
}

/// 文本清理工具
pub struct TextCleaner;

impl TextCleaner {
    /// 清理多余的空白字符
    pub fn clean_whitespace(text: &str) -> String {
        regex::Regex::new(r"\s+")
            .unwrap()
            .replace_all(text.trim(), " ")
            .to_string()
    }
    
    /// 移除控制字符
    pub fn remove_control_chars(text: &str) -> String {
        text.chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
            .collect()
    }
    
    /// 标准化换行符
    pub fn normalize_line_breaks(text: &str) -> String {
        text.replace("\r\n", "\n").replace('\r', "\n")
    }
    
    /// 完整清理流程
    pub fn clean_text(text: &str) -> String {
        let cleaned = Self::remove_control_chars(text);
        let normalized = Self::normalize_line_breaks(&cleaned);
        Self::clean_whitespace(&normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_cleaner() {
        let dirty_text = "  Hello   \r\n  World  \t  ";
        let cleaned = TextCleaner::clean_text(dirty_text);
        assert_eq!(cleaned, "Hello World");
    }

    #[test]
    fn test_normalize_line_breaks() {
        let text_with_crlf = "Line1\r\nLine2\rLine3\nLine4";
        let normalized = TextCleaner::normalize_line_breaks(text_with_crlf);
        assert_eq!(normalized, "Line1\nLine2\nLine3\nLine4");
    }
}