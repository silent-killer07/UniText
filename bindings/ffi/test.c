#include <stdio.h>
#include <stdlib.h>
#include "unitext.h"

int main() {
    printf("Testing UniText C FFI bindings...\n\n");

    const char* test_str = "Hello 👨‍👩‍👧‍👦 Café";
    
    // Test analyze
    char* analysis = unitext_analyze(test_str);
    if (analysis) {
        printf("Analysis of '%s':\n%s\n\n", test_str, analysis);
        unitext_free_string(analysis);
    }
    
    // Test security
    const char* safe = "apple.com";
    const char* unsafe = "аpple.com"; // Cyrillic 'a'
    
    char* safe_res = unitext_is_safe(safe);
    char* unsafe_res = unitext_is_safe(unsafe);
    
    if (safe_res && unsafe_res) {
        printf("Safe string risk: %s\n", safe_res);
        printf("Unsafe string risk: %s\n\n", unsafe_res);
        unitext_free_string(safe_res);
        unitext_free_string(unsafe_res);
    }
    
    // Test equality
    int eq = unitext_visually_equal(safe, unsafe);
    printf("Are '%s' and '%s' visually equal? %s\n\n", safe, unsafe, eq ? "Yes" : "No");
    
    // Test to_ascii
    char* ascii = unitext_to_ascii("Héllo");
    if (ascii) {
        printf("ASCII conversion of 'Héllo': %s\n", ascii);
        unitext_free_string(ascii);
    }

    return 0;
}
