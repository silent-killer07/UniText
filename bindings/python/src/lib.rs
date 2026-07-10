use pyo3::prelude::*;
use serde_json::json;
use unitext_core::normalizer::Normalizer;
use unitext_security::{assess_risk, RiskLevel};
use unitext_string::UniString;

/// Analyzes the text and returns a JSON string with the analysis results.
#[pyfunction]
fn analyze(text: &str) -> PyResult<String> {
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
        "graphemesCount": table.graphemes.len(),
        "codePointsCount": text.chars().count(),
        "utf8Bytes": text.len(),
        "script": dominant_script,
        "isMixedScript": is_mixed,
        "graphemeBreakdown": graphemes
    });

    Ok(result.to_string())
}

/// Checks if the text is safe from homograph attacks and mixed scripts.
#[pyfunction]
fn is_safe(text: &str) -> PyResult<String> {
    let table = Normalizer::process(text);

    let mut text_only = String::new();
    for g in &table.graphemes {
        text_only.push_str(&g.canonical_form);
    }

    let risk = assess_risk(&text_only, &table);

    let (safe, score, level) = match risk {
        RiskLevel::None => (true, 0, "None"),
        RiskLevel::Low => (true, 25, "Low"),
        RiskLevel::Medium => (false, 75, "Medium"),
        RiskLevel::High => (false, 100, "High"),
    };

    let result = json!({
        "safe": safe,
        "riskScore": score,
        "level": level
    });

    Ok(result.to_string())
}

/// Compares two strings for visual equality.
#[pyfunction]
fn visually_equal(text1: &str, text2: &str) -> PyResult<bool> {
    Ok(UniString::visually_equal(text1, text2))
}

/// Converts the text to ASCII.
#[pyfunction]
fn to_ascii(text: &str) -> PyResult<String> {
    let us = UniString::new(text);
    let (output, _lossy) = us.to_ascii();
    Ok(output)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn unitext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze, m)?)?;
    m.add_function(wrap_pyfunction!(is_safe, m)?)?;
    m.add_function(wrap_pyfunction!(visually_equal, m)?)?;
    m.add_function(wrap_pyfunction!(to_ascii, m)?)?;
    Ok(())
}
