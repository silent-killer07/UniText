use std::collections::HashMap;
use std::sync::OnceLock;

static CONFUSABLES_DATA: &str = include_str!("../data/confusables.txt");
static CONFUSABLES_MAP: OnceLock<HashMap<char, char>> = OnceLock::new();

pub fn init_confusables() {
    CONFUSABLES_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        for line in CONFUSABLES_DATA.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() >= 2 {
                let source = u32::from_str_radix(parts[0].trim(), 16).ok();
                let target = u32::from_str_radix(parts[1].trim(), 16).ok();

                if let (Some(sc), Some(tc)) = (
                    source.and_then(char::from_u32),
                    target.and_then(char::from_u32),
                ) {
                    map.insert(sc, tc);
                }
            }
        }
        map
    });
}

pub fn get_confusable(c: char) -> Option<char> {
    init_confusables();
    CONFUSABLES_MAP.get().unwrap().get(&c).copied()
}

pub fn normalize_confusables(input: &str) -> String {
    input
        .chars()
        .map(|c| get_confusable(c).unwrap_or(c))
        .collect()
}
