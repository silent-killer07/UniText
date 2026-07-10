import unitext
import json

def test_unitext():
    print("Testing unitext-python bindings...")
    
    # Test analyze
    print("\n--- Analyze ---")
    text = "Hello 👨‍👩‍👧‍👦 Café"
    analysis_json = unitext.analyze(text)
    analysis = json.loads(analysis_json)
    print("Text parsed successfully.")
    print(f"Graphemes Count: {analysis['graphemesCount']}")
    print(f"Code Points Count: {analysis['codePointsCount']}")
    print(f"UTF-8 Bytes: {analysis['utf8Bytes']}")
    print(f"First Grapheme: {analysis['graphemeBreakdown'][0]['char']}")
    
    # Test is_safe
    print("\n--- Security Check ---")
    safe_text = "apple.com"
    unsafe_text = "аpple.com" # Cyrillic 'a'
    
    safe_json = unitext.is_safe(safe_text)
    unsafe_json = unitext.is_safe(unsafe_text)
    
    print(f"Safe text Risk Level: {json.loads(safe_json)['level']}")
    print(f"Unsafe text Risk Level: {json.loads(unsafe_json)['level']}")
    
    # Test visually_equal
    print("\n--- Visual Equality ---")
    is_equal = unitext.visually_equal(safe_text, unsafe_text)
    print(f"Are they visually equal? {is_equal}")
    
    # Test to_ascii
    print("\n--- ASCII Conversion ---")
    lossy_text = "Héllo"
    ascii_text = unitext.to_ascii(lossy_text)
    print(f"ASCII Output: {ascii_text}")

if __name__ == "__main__":
    test_unitext()
