use unicode_normalization::UnicodeNormalization;

fn is_combining_mark(c: char) -> bool {
    let cp = c as u32;
    matches!(cp,
        0x0300..=0x036F | // Combining Diacritical Marks
        0x1AB0..=0x1AFF | // Combining Diacritical Marks Extended
        0x1DC0..=0x1DFF | // Combining Diacritical Marks Supplement
        0x20D0..=0x20FF | // Combining Diacritical Marks for Symbols
        0xFE20..=0xFE2F   // Combining Half Marks
    )
}

fn apply_ligatures(c: char) -> Option<&'static str> {
    match c {
        'ß' => Some("ss"),
        'æ' => Some("ae"),
        'Æ' => Some("Ae"),
        'ø' => Some("o"),
        'Ø' => Some("O"),
        'đ' => Some("d"),
        'Đ' => Some("D"),
        'ł' => Some("l"),
        'Ł' => Some("L"),
        'œ' => Some("oe"),
        'Œ' => Some("Oe"),
        _ => None,
    }
}

pub fn to_ascii(text: &str) -> (String, bool) {
    let mut result = String::new();
    let mut lossy = false;

    for c in text.nfkd() {
        if c.is_ascii() {
            result.push(c);
        } else if let Some(ligature) = apply_ligatures(c) {
            result.push_str(ligature);
            lossy = true;
        } else if is_combining_mark(c) {
            lossy = true;
        } else {
            result.push('?');
            lossy = true;
        }
    }

    (result, lossy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ascii() {
        let (ascii, lossy) = to_ascii("Hello");
        assert_eq!(ascii, "Hello");
        assert!(!lossy);

        let (ascii, lossy) = to_ascii("Café");
        assert_eq!(ascii, "Cafe");
        assert!(lossy);

        let (ascii, lossy) = to_ascii("naïve");
        assert_eq!(ascii, "naive");
        assert!(lossy);

        let (ascii, lossy) = to_ascii("Straße");
        assert_eq!(ascii, "Strasse");
        assert!(lossy);
        
        let (ascii, lossy) = to_ascii("👨‍👩‍👧‍👦");
        assert_eq!(ascii, "???????");
        assert!(lossy);
    }
}
