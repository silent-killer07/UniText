use unitext_core::grapheme_table::GraphemeTable;
use unitext_core::normalizer::Normalizer;

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
}
