#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphemeEntry {
    pub canonical_form: String,
    pub script: String,
    pub category: String,
    pub visual_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphemeTable {
    pub graphemes: Vec<GraphemeEntry>,
    pub visuals: Vec<String>,
}

impl Default for GraphemeTable {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphemeTable {
    pub fn new() -> Self {
        Self {
            graphemes: Vec::new(),
            visuals: Vec::new(),
        }
    }

    pub fn push(&mut self, entry: GraphemeEntry) {
        self.graphemes.push(entry);
    }
}
