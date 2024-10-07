import mkdocs_gen_files
import os
import re

def split_md_file(input_file):
    with open(input_file, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    current_file = None
    file_count = 0

    for line in lines:
        if line.startswith("# "):  # Split by "##" headers
            # Close previous file if it's open
            if current_file:
                current_file.close()

            # Create a new file for the section
            header = line.strip().replace("# ", "")
            file_count += 1
            new_file_name = f"lua/{file_count}_{header}.md"
            invalid_chars = r'[<>:"/\\|?*]'
            new_file_name = re.sub(invalid_chars, '', new_file_name)

            current_file = mkdocs_gen_files.open(new_file_name, 'w', encoding='utf-8')
            mkdocs_gen_files.set_edit_path(new_file_name, "gen_pages.py")
        
        if current_file:
            current_file.write(line)

    # Close the last file
    if current_file:
        current_file.close()

split_md_file("./docs/doc.md")