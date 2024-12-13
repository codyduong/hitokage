import mkdocs_gen_files  # type: ignore
import re
import urllib.parse


IGNORE = (
    "any", 
    "arg", 
    "assert",
    "boolean",
    "collectgarbage",
    "coroutine",
    "debug",
    "debuglib",
    "debuginfo",
    "dofile",
    "error",
    "exitcode",
    "false",
    "file*",
    "filetype",
    "function",
    "gcoptions",
    "getfenv",
    "getmetatable",
    "hookmask",
    "infowhat",
    "integer",
    "io",
    "iolib",
    "ipairs",
    "lightuserdata",
    "load",
    "loadfile",
    "loadmode",
    "loadstring",
    "localecategory",
    "math",
    "mathlib",
    "metatable",
    "module",
    "newproxy",
    "next",
    "nil",
    "number",
    "openmode",
    "os",
    "osdate",
    "osdateparam",
    "os",
    "oslib",
    "package",
    "packagelib",
    "pairs",
    "pcall",
    "popenmode",
    "print",
    "rawequal",
    "rawget",
    "rawlen",
    "rawset",
    "readmode",
    "require",
    "seekwhence",
    "select",
    "setfenv",
    "setmetatable",
    "string",
    "stringlib",
    "table",
    "tablelib",
    "thread",
    "tonumber",
    "tostring",
    "true",
    "type",
    "unknown",
    "unpack",
    "userdata",
    "utf8",
    "utf8lib",
    "vbuf",
    "warn",
    "xpcall",
    )


header_to_content: dict[str, str] = {}


def process_line(line: str) -> str:
    # Regex pattern to find markdown links: [link_text](url)
    pattern = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")

    # Function to process each found link
    def replace_link(match: re.Match[str]) -> str:
        link_text: str = match.group(1)
        url: str = match.group(2)

        if url.startswith("file://"):
            # Remove 'file://'
            file_url = url[len("file://") :]

            # URL-decode the path (e.g., c%3A to c:)
            file_path = urllib.parse.unquote(file_url)

            # Remove any leading slashes
            file_path = file_path.lstrip("/")

            # Extract the base filename
            # base_filename = os.path.basename(file_path)

            # Remove the file extension
            # base_name, _ = os.path.splitext(base_filename)

            # Use the link text as the anchor
            anchor = link_text

            new_url = f"#{anchor}"

            if anchor.count(':') == 1:
                path, anchor = anchor.split(":")
                new_url = f"./{path}.html#{anchor}"

            return f"[{link_text}]({new_url})"
        else:
            # Return the original match if not a 'file://' link
            return match.group(0)

    # Substitute all markdown links in the line using the replace_link function
    new_line = pattern.sub(replace_link, line)
    return new_line


def split_md_file(input_file: str):
    with open(input_file, "r", encoding="utf-8") as f:
        lines = f.readlines()

    current_file_name: None | str = None

    try:
        for line in lines:
            if line.startswith("# "):  # Split by "# " headers
                header = line.strip().replace("# ", "")

                # ignore things in the IGNORE tuple or start with _
                if header.startswith("_") or header in IGNORE:
                    print(f"Skipping generation of: {header}")
                    current_file_name = None
                    continue

                # we don't want to have foo.bar, this should be nested under foo > bar
                if "." in header:
                    skip = True
                    # for ignore in IGNORE:
                    #     if re.match(f"{ignore}\\..+", header):
                    #         skip = True
                    #         break
                    if skip:
                        print(f"Skipping generation of: {header}")
                        current_file_name = None
                        continue

                # Close previous file if it's open
                if current_file_name:
                    current_file_name = None

                # Create a new file for the section
                invalid_chars = r'[<>:"/\\|?*]'
                header = re.sub(invalid_chars, "", header).lower()
                new_file_name = f"lua/{header}.md"
                current_file_name = new_file_name

            if current_file_name:
                # Process the line to replace links
                processed_line = process_line(line)
                if current_file_name in header_to_content:
                    header_to_content[current_file_name] += f"{processed_line}"
                else:
                    header_to_content[current_file_name] = f"{processed_line}"
    finally:
        for name, content in header_to_content.items():
            with mkdocs_gen_files.open(name, "w", encoding="utf-8") as f: # type: ignore
                f.writelines(content) # type: ignore
            mkdocs_gen_files.set_edit_path(name, "gen_pages.py")  # type: ignore


split_md_file("./docs/doc.md")
