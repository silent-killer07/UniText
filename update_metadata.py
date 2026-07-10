import os
import glob

metadata = """authors = ["Divya Jaiswal"]
description = "A Next-Generation Text Encoding Abstraction System"
license = "MIT OR Apache-2.0"
repository = "https://github.com/silent-killer07/UniText"
keywords = ["unicode", "text", "security", "grapheme"]
"""

def process_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    if "authors = " in content:
        print(f"Skipping {filepath}, metadata already exists.")
        return

    lines = content.split('\n')
    new_lines = []
    inserted = False
    for line in lines:
        new_lines.append(line)
        if line.startswith('edition = ') and not inserted:
            new_lines.append(metadata.strip())
            inserted = True
            
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write('\n'.join(new_lines))
    print(f"Updated {filepath}")

# Find all Cargo.toml files in crates and bindings
for pattern in ['crates/*/Cargo.toml', 'bindings/*/Cargo.toml']:
    for filepath in glob.glob(pattern):
        process_file(filepath)
