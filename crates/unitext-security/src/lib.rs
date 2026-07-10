pub mod confusables;
pub mod homograph;
pub mod mixed_script;
pub mod risk_scorer;

pub use confusables::{get_confusable, normalize_confusables};
pub use homograph::visually_equal;
pub use mixed_script::detect_mixed_script;
pub use risk_scorer::{assess_risk, RiskLevel};
