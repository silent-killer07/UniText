use crate::confusables::normalize_confusables;

pub fn visually_equal(a: &str, b: &str) -> bool {
    // Basic implementation: normalize confusables and compare
    let norm_a = normalize_confusables(a);
    let norm_b = normalize_confusables(b);

    norm_a == norm_b
}
