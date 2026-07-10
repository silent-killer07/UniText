use wasm_bindgen::prelude::*;
use unitext_string::UniString;
use unitext_core::normalizer::Normalizer;
use unitext_security::assess_risk;
use serde_json::json;

#[wasm_bindgen]
pub struct UniText;

#[wasm_bindgen]
impl UniText {
    pub fn analyze(text: &str) -> String {
        let table = Normalizer::process(text);
        let mut graphemes = Vec::new();
        
        let mut dominant_script = "Unknown".to_string();
        let mut is_mixed = false;
        let mut script_counts = std::collections::HashMap::new();

        for (i, g) in table.graphemes.iter().enumerate() {
            let char_str = if let Some(vid) = g.visual_id {
                table.visuals[vid].clone()
            } else {
                g.canonical_form.clone()
            };
            
            if g.script != "Unknown" && g.script != "Common" && g.script != "Inherited" {
                *script_counts.entry(g.script.clone()).or_insert(0) += 1;
            }

            graphemes.push(json!({
                "slot": i,
                "char": char_str,
                "script": g.script,
                "category": g.category,
            }));
        }
        
        if script_counts.len() > 1 {
            is_mixed = true;
        }
        if let Some((s, _)) = script_counts.into_iter().max_by_key(|&(_, count)| count) {
            dominant_script = s;
        }

        let result = json!({
            "input": text,
            "graphemes_count": table.graphemes.len(),
            "code_points_count": text.chars().count(),
            "bytes_count": text.len(),
            "script": dominant_script,
            "is_mixed_script": is_mixed,
            "grapheme_breakdown": graphemes
        });
        
        result.to_string()
    }
    
    pub fn is_safe(text: &str) -> String {
        let table = Normalizer::process(text);
        
        let mut text_only = String::new();
        for g in &table.graphemes {
            text_only.push_str(&g.canonical_form);
        }
        
        let risk = assess_risk(&text_only, &table);
        
        let (safe, score, details) = match risk {
            unitext_security::RiskLevel::None => (true, 0, "Clean"),
            unitext_security::RiskLevel::Low => (true, 25, "Low risk (Unusual characters)"),
            unitext_security::RiskLevel::Medium => (false, 75, "Medium risk (Confusable characters)"),
            unitext_security::RiskLevel::High => (false, 100, "High risk (Mixed-script homograph attack)"),
        };
        
        let result = json!({
            "safe": safe,
            "risk_score": score,
            "details": details
        });
        
        result.to_string()
    }
    
    pub fn visually_equal(text1: &str, text2: &str) -> bool {
        UniString::visually_equal(text1, text2)
    }
    
    pub fn to_ascii(text: &str) -> String {
        let us = UniString::new(text);
        let (output, lossy) = us.to_ascii();
        let result = json!({
            "output": output,
            "lossy": lossy
        });
        
        result.to_string()
    }
}
