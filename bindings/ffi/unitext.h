#ifndef UNITEXT_H
#define UNITEXT_H

#ifdef __cplusplus
extern "C" {
#endif

// Analyzes the text and returns a JSON string with the analysis results.
// The caller is responsible for freeing the returned string using `unitext_free_string`.
char* unitext_analyze(const char* text);

// Checks if the text is safe from homograph attacks and mixed scripts.
// Returns a JSON string with the security risk analysis.
// The caller is responsible for freeing the returned string using `unitext_free_string`.
char* unitext_is_safe(const char* text);

// Compares two strings for visual equality.
// Returns 1 if visually equal, 0 if not.
int unitext_visually_equal(const char* text1, const char* text2);

// Converts the text to ASCII.
// The caller is responsible for freeing the returned string using `unitext_free_string`.
char* unitext_to_ascii(const char* text);

// Frees a string previously returned by a unitext function.
void unitext_free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif // UNITEXT_H
