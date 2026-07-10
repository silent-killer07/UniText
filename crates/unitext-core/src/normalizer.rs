use crate::grapheme_table::{GraphemeEntry, GraphemeTable};
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

pub struct Normalizer;

impl Normalizer {
    pub fn process(input: &str) -> GraphemeTable {
        // 1. Decode (Input is already a string slice, assuming UTF-8 or decoded)

        // 2. Normalize (NFC)
        let normalized: String = input.nfc().collect();

        // 3. Segment (UAX #29 grapheme clusters)
        let graphemes = normalized.graphemes(true);

        let mut table = GraphemeTable::new();

        for g in graphemes {
            // 4. Classify (Stub: In Phase 2, use icu4x)
            let script = "Unknown".to_string();
            let category = "Unknown".to_string();

            // 5. Separate Visual (Stub: extracting basic emojis)
            let (canonical_form, visual_id) = if is_emoji(g) {
                let id = table.visuals.len();
                table.visuals.push(g.to_string());
                (String::new(), Some(id))
            } else {
                (g.to_string(), None)
            };

            // 6. Index (Add to table)
            table.push(GraphemeEntry {
                canonical_form,
                script,
                category,
                visual_id,
            });
        }

        table
    }
}

fn is_emoji(g: &str) -> bool {
    g.chars().any(|c| {
        let code = c as u32;
        // Simple heuristic for emoji ranges
        (0x1F300..=0x1F9FF).contains(&code)
            || (0x1FA70..=0x1FAFF).contains(&code)
            || (code == 0x200D) // ZWJ
    })
}
