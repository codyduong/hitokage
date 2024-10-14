import mkdocs_gen_files
import os
import re
import urllib.parse


def process_line(line):
    # Regex pattern to find markdown links: [link_text](url)
    pattern = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")

    # Function to process each found link
    def replace_link(match):
        link_text = match.group(1)
        url = match.group(2)

        if url.startswith("file://"):
            # Remove 'file://'
            file_url = url[len("file://") :]

            # URL-decode the path (e.g., c%3A to c:)
            file_path = urllib.parse.unquote(file_url)

            # Remove any leading slashes
            file_path = file_path.lstrip("/")

            # Extract the base filename
            base_filename = os.path.basename(file_path)

            # Remove the file extension
            base_name, _ = os.path.splitext(base_filename)

            # Use the link text as the anchor
            anchor = link_text

            # Construct the new URL
            new_url = f"./{base_name}.html#{anchor}"

            # Return the transformed link
            return f"[{link_text}]({new_url})"
        else:
            # Return the original match if not a 'file://' link
            return match.group(0)

    # Substitute all markdown links in the line using the replace_link function
    new_line = pattern.sub(replace_link, line)
    return new_line


def split_md_file(input_file):
    with open(input_file, "r", encoding="utf-8") as f:
        lines = f.readlines()

    current_file = None

    for line in lines:
        if line.startswith("# "):  # Split by "# " headers
            # Close previous file if it's open
            if current_file:
                current_file.close()

            # Create a new file for the section
            header = line.strip().replace("# ", "")
            invalid_chars = r'[<>:"/\\|?*]'
            header = re.sub(invalid_chars, "", header)
            new_file_name = f"lua/{header}.md"

            current_file = mkdocs_gen_files.open(new_file_name, "w", encoding="utf-8")
            mkdocs_gen_files.set_edit_path(new_file_name, "gen_pages.py")

        if current_file:
            # Process the line to replace links
            processed_line = process_line(line)
            current_file.write(processed_line)

    # Close the last file
    if current_file:
        current_file.close()


split_md_file("./docs/doc.md")
