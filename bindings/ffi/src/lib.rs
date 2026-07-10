use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use unitext_core::normalizer::Normalizer;
use unitext_security::{assess_risk, RiskLevel};
use unitext_string::UniString;

/// Analyzes the text and returns a JSON string with the analysis results.
/// The caller is responsible for freeing the returned string using `unitext_free_string`.
#[no_mangle]
pub extern "C" fn unitext_analyze(text: *const c_char) -> *mut c_char {
    if text.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let us = UniString::new(text_str);

    let result = format!("{{\"length\": {}}}", us.length()); // Simplified for C FFI

    match CString::new(result) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Checks if the text is safe from homograph attacks and mixed scripts.
/// Returns a JSON string with the security risk analysis.
/// The caller is responsible for freeing the returned string using `unitext_free_string`.
#[no_mangle]
pub extern "C" fn unitext_is_safe(text: *const c_char) -> *mut c_char {
    if text.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let table = Normalizer::process(text_str);
    let mut text_only = String::new();
    for g in &table.graphemes {
        text_only.push_str(&g.canonical_form);
    }

    let risk = assess_risk(&text_only, &table);

    let level_str = match risk {
        RiskLevel::None => "None",
        RiskLevel::Low => "Low",
        RiskLevel::Medium => "Medium",
        RiskLevel::High => "High",
    };

    let result = format!("{{\"level\": \"{}\"}}", level_str);

    match CString::new(result) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Compares two strings for visual equality.
/// Returns 1 if visually equal, 0 if not.
#[no_mangle]
pub extern "C" fn unitext_visually_equal(text1: *const c_char, text2: *const c_char) -> i32 {
    if text1.is_null() || text2.is_null() {
        return 0;
    }

    let c_str1 = unsafe { CStr::from_ptr(text1) };
    let c_str2 = unsafe { CStr::from_ptr(text2) };

    let (t1, t2) = match (c_str1.to_str(), c_str2.to_str()) {
        (Ok(s1), Ok(s2)) => (s1, s2),
        _ => return 0,
    };

    if UniString::visually_equal(t1, t2) {
        1
    } else {
        0
    }
}

/// Converts the text to ASCII.
/// The caller is responsible for freeing the returned string using `unitext_free_string`.
#[no_mangle]
pub extern "C" fn unitext_to_ascii(text: *const c_char) -> *mut c_char {
    if text.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let uni_str = UniString::new(text_str);
    let (output, _lossy) = uni_str.to_ascii();

    match CString::new(output) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees a string previously returned by a unitext function.
#[no_mangle]
pub extern "C" fn unitext_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}
