use unitext_string::UniString;

/// A simple URL validator that checks for Homograph attacks
fn validate_url(url: &str) -> Result<(), &'static str> {
    let us = UniString::new(url);

    // 1. Check if the URL contains dangerous mixed scripts or confusables
    if !us.is_safe() {
        return Err("URL contains suspicious characters or mixed scripts (Homograph Attack Risk)");
    }

    // 2. Explicitly check against known protected domains
    let protected_domains = ["apple.com", "google.com", "paypal.com"];
    for domain in protected_domains {
        // If it's not byte-equal but is visually equal, it's an attack
        if url != domain && UniString::visually_equal(url, domain) {
            return Err("URL is a homograph spoof of a protected domain!");
        }
    }

    Ok(())
}

fn main() {
    let safe_url = "apple.com";
    let spoof_url = "аpple.com"; // Cyrillic 'a'

    println!("Validating {}: {:?}", safe_url, validate_url(safe_url));
    println!("Validating {}: {:?}", spoof_url, validate_url(spoof_url));
}
