use unitext_core::grapheme_table::GraphemeTable;
use crate::mixed_script::detect_mixed_script;
use crate::confusables::normalize_confusables;

#[derive(Debug, PartialEq)]
pub enum RiskLevel {
    None,
    Low,
    Medium,
    High,
}

pub fn assess_risk(original: &str, table: &GraphemeTable) -> RiskLevel {
    let mixed = detect_mixed_script(table);
    let normalized = normalize_confusables(original);
    let has_confusables = original != normalized;
    
    if mixed && has_confusables {
        RiskLevel::High
    } else if mixed || has_confusables {
        RiskLevel::Medium
    } else {
        RiskLevel::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use unitext_core::normalizer::Normalizer;

    #[test]
    fn test_risk_scorer() {
        let safe_text = "apple.com";
        let table = Normalizer::process(safe_text);
        assert_eq!(assess_risk(safe_text, &table), RiskLevel::None);

        let homograph_text = "аpple.com"; // Cyrillic 'a'
        let table = Normalizer::process(homograph_text);
        assert_eq!(assess_risk(homograph_text, &table), RiskLevel::High);
    }
}
