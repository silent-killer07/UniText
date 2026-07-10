use unitext_core::grapheme_table::GraphemeTable;

pub fn detect_mixed_script(table: &GraphemeTable) -> bool {
    let mut scripts = Vec::new();

    // In Phase 1 we stubbed script classification.
    // For this demonstration, we'll infer script loosely from character blocks.
    for entry in &table.graphemes {
        let canonical = &entry.canonical_form;
        if let Some(c) = canonical.chars().next() {
            let script = guess_script(c);
            if script != "Common" && script != "Emoji" && !scripts.contains(&script) {
                scripts.push(script);
            }
        }
    }

    // If there's more than one distinct script (excluding Common/Emoji), it's mixed.
    scripts.len() > 1
}

fn guess_script(c: char) -> String {
    let code = c as u32;
    if (0x0041..=0x02AF).contains(&code) {
        "Latin".to_string()
    } else if (0x0400..=0x052F).contains(&code) {
        "Cyrillic".to_string()
    } else if (0x0370..=0x03FF).contains(&code) {
        "Greek".to_string()
    } else if (0x1F300..=0x1F9FF).contains(&code) || (0x1FA70..=0x1FAFF).contains(&code) {
        "Emoji".to_string()
    } else {
        "Common".to_string()
    }
}
