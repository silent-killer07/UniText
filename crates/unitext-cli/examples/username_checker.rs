use unitext_string::UniString;

/// Validates a username for registration
fn validate_username(username: &str) -> Result<(), &'static str> {
    let us = UniString::new(username);

    // 1. Length bounds (checking human characters, not bytes!)
    if us.length() < 3 || us.length() > 20 {
        return Err("Username must be between 3 and 20 characters.");
    }

    // 2. Ensure the username is completely safe (no mixed scripts, no weird symbols)
    if !us.is_safe() {
        return Err("Username contains invalid or suspicious characters.");
    }

    Ok(())
}

fn main() {
    let valid_user = "cool_dev99";
    let invalid_user = "cool_dеv99"; // Cyrillic 'e'
    let emoji_user = "👨‍👩‍👧‍👦_fan";

    println!(
        "Registering {}: {:?}",
        valid_user,
        validate_username(valid_user)
    );
    println!(
        "Registering {}: {:?}",
        invalid_user,
        validate_username(invalid_user)
    );
    println!(
        "Registering {}: {:?}",
        emoji_user,
        validate_username(emoji_user)
    );
}
