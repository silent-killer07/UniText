use unitext_string::UniString;

/// Sanitizes a bio for a legacy system that only accepts ASCII
fn sanitize_for_legacy_system(input: &str) -> String {
    let us = UniString::new(input);
    
    // Automatically normalizes, strips invisible marks, and safely transliterates
    let (ascii_output, is_lossy) = us.to_ascii();

    if is_lossy {
        println!("Note: Some characters were lost or transliterated during ASCII conversion.");
    }

    ascii_output
}

fn main() {
    let input = "Héllo, my name is naïve café and I love 👨‍👩‍👧‍👦!";
    let sanitized = sanitize_for_legacy_system(input);
    
    println!("Original:  {}", input);
    println!("Sanitized: {}", sanitized);
}
