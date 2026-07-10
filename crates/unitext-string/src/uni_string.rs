use unitext_core::grapheme_table::GraphemeTable;
use unitext_core::normalizer::Normalizer;
use unitext_security::{assess_risk, visually_equal, RiskLevel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniString {
    table: GraphemeTable,
}

impl UniString {
    pub fn new(input: &str) -> Self {
        Self {
            table: Normalizer::process(input),
        }
    }

    pub fn length(&self) -> usize {
        self.table.graphemes.len()
    }

    pub fn char_at(&self, index: usize) -> Option<String> {
        self.table.graphemes.get(index).map(|entry| {
            if let Some(vid) = entry.visual_id {
                self.table.visuals.get(vid).cloned().unwrap_or_default()
            } else {
                entry.canonical_form.clone()
            }
        })
    }

    pub fn reverse(&self) -> String {
        let mut result = String::new();
        for i in (0..self.length()).rev() {
            if let Some(c) = self.char_at(i) {
                result.push_str(&c);
            }
        }
        result
    }

    pub fn substring(&self, start: usize, end: usize) -> Option<String> {
        if start > end || end > self.length() {
            return None;
        }
        let mut result = String::new();
        for i in start..end {
            if let Some(c) = self.char_at(i) {
                result.push_str(&c);
            }
        }
        Some(result)
    }

    pub fn text_only(&self) -> String {
        let mut result = String::new();
        for g in &self.table.graphemes {
            result.push_str(&g.canonical_form);
        }
        result
    }

    pub fn is_safe(&self) -> bool {
        let text = self.text_only();
        assess_risk(&text, &self.table) == RiskLevel::None
    }

    pub fn visually_equal(a: &str, b: &str) -> bool {
        visually_equal(a, b)
    }

    pub fn to_utf8(&self) -> Vec<u8> {
        self.text_only().into_bytes()
    }

    pub fn to_utf32(&self) -> Vec<char> {
        self.text_only().chars().collect()
    }

    pub fn to_ascii(&self) -> (String, bool) {
        crate::convert::to_ascii(&self.text_only())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uni_string_length() {
        let text = UniString::new("hello");
        assert_eq!(text.length(), 5);
        
        let emoji = UniString::new("👨‍👩‍👧‍👦");
        assert_eq!(emoji.length(), 1);
    }

    #[test]
    fn test_uni_string_security() {
        let safe = UniString::new("apple.com");
        assert!(safe.is_safe());

        let homograph = UniString::new("аpple.com"); // Cyrillic 'a'
        assert!(!homograph.is_safe());

        assert!(UniString::visually_equal("apple.com", "аpple.com"));
    }
}
