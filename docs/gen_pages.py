import html
import os
import mkdocs_gen_files  # type: ignore
import json
import re
import urllib.parse

# generated from `lua-language-server --doc "hitokage-lua-lib" --doc_out_path "docs"`
DOC_JSON = "./docs/doc.json"
DOC_SCHEMA = "./docs/doc_schema.json"
LUA_NATIVE_TYPES = (
    "nil",
    "boolean",
    "number",
    "string",
    "userdata",
    "function",
    "thread",
    "table",
    "integer",
)
REPO_URL = "https://github.com/codyduong/hitokage"
REPO_NAME = "hitokage"
PERMALINK_ICON = "¶"

from typing import (
    Annotated,
    Callable,
    List,
    Literal,
    Optional,
    TextIO,
    Tuple,
    Union,
    cast,
)
from pydantic import BaseModel, Field, HttpUrl, RootModel, conlist

# non comprehensive list of types
# https://github.com/LuaLS/lua-language-server/blob/108ce76c99bcb9990421edd4453a2ca8e282af95/script/cli/doc/export.lua#L12C1-L33C12


nav = mkdocs_gen_files.Nav() # type: ignore


# with mkdocs_gen_files.open("SUMMARY.md", "w") as nav_file:
#     print([s for s in nav.build_literate_nav()])
#     nav_file.writelines(nav.build_literate_nav())


class FieldDefinition(BaseModel):
    async_: Optional[bool] = Field(alias="async")
    deprecated: Optional[bool] = None
    desc: Optional[str] = None
    rawdesc: Optional[str] = None
    file: Optional[str] = None  # Optional[HttpUrl]
    start: Annotated[list[int], 2] = conlist(int, min_length=2, max_length=2)
    finish: Annotated[list[int], 2] = conlist(int, min_length=2, max_length=2)
    name: Optional[str] = None
    type: str
    view: str
    visible: Optional[str] = None
    extends: "Extends"

    class Config:
        allow_population_by_field_name = True


class DefineDefinition(BaseModel):
    async_: Optional[bool] = Field(alias="async", default=None)
    deprecated: Optional[bool] = None
    desc: Optional[str] = None
    rawdesc: Optional[str] = None
    file: Optional[str] = None  # Optional[HttpUrl]
    start: Annotated[list[int], 2] = conlist(int, min_length=2, max_length=2)
    finish: Annotated[list[int], 2] = conlist(int, min_length=2, max_length=2)
    name: Optional[str] = None
    type: str
    view: str
    extends: Optional["Extends"] = None

    class Config:
        allow_population_by_field_name = True


class TypesDefinition(BaseModel):
    desc: Optional[str] = None
    rawdesc: Optional[str] = None
    finish: Annotated[list[int], 2] = conlist(int, min_length=2, max_length=2)
    start: Annotated[list[int], 2] = conlist(int, min_length=2, max_length=2)
    name: Optional[str] = None
    type: str
    view: str


class ExtendsFunction(BaseModel):
    args: List['TypesDefinition']
    desc: Optional[str] = None
    rawdesc: Optional[str] = None
    finish: Annotated[list[int], 2] = conlist(int, min_length=2, max_length=2)
    start: Annotated[list[int], 2] = conlist(int, min_length=2, max_length=2)
    type: Literal["function"]
    view: str
    returns: Optional[List[TypesDefinition]] = []


Extends = Union[ExtendsFunction, TypesDefinition, List[TypesDefinition]]


# The root item in the array
class RootItem(BaseModel):
    defines: List[DefineDefinition]
    fields: Optional[List[FieldDefinition]] = None
    name: str
    type: str
    view: str


# Root array model
class Root(RootModel[List[RootItem]]):
    pass


data: None | Root = None


with open(DOC_JSON, "r", encoding="utf8") as raw_str:
    data = Root(json.load(raw_str))


mkdocs_open = cast(Callable[..., TextIO], mkdocs_gen_files.open)


# view to url
links: dict[str, str] = {
    'nil':'https://www.lua.org/pil/2.1.html',
    'boolean': 'https://www.lua.org/pil/2.2.html',
    'number': 'https://www.lua.org/pil/2.3.html',
    'string': 'https://www.lua.org/pil/2.4.html',
    'userdata': 'https://www.lua.org/pil/28.1.html',
    'function': 'https://www.lua.org/pil/2.6.html',
    'thread': 'https://www.lua.org/pil/9.html',
    'table': 'https://www.lua.org/pil/2.5.html',
    'integer': 'https://www.lua.org/pil/2.3.html',
}
def register_link(view: str, path: str) -> None:
    links[view] = f"/{REPO_NAME}/{path}"


def process_view(view: str) -> str:
    # Handle optional type with `?`
    if view.endswith('?'):
        base_type: str = view[:-1].strip()
        return f'{process_view(base_type)}<a href="{links['nil']}">?</a>'

    # Handle unions (split by | and process each part)
    if "|" in view:
        parts = re.split(r'(\s*\|\s*)', view)  # Preserve spacers
        return ''.join(process_view(part) if part.strip() != '|' else part for part in parts)
    
    # Handle generic types (e.g., table<number, string>)
    generic_match = re.match(r'(\w+)\s*<(.+)>', view)
    if generic_match:
        base_type = generic_match.group(1)
        args = generic_match.group(2)
        if base_type not in links:
            raise ValueError(f"Unknown type: {base_type}")
        # Process the base type and arguments
        linked_base = f"[{base_type}]({links[base_type]})"
        linked_args = ', '.join(process_view(arg.strip()) for arg in split_generics(args))
        return f"{linked_base}<{linked_args}>"
    
    # Handle string literals wrapped in single quotes
    if re.fullmatch(r"'[^']*'", view.strip()):
        return f"""<a href="{links['string']}">{view}</a>"""
    
    # Handle number literals
    if re.fullmatch(r'\d+', view.strip()):
        return f"""<a href="{links['number']}">{view}</a>"""
    
    # Handle primitive/custom types
    view = view.strip()
    if view in links:
        return f'<a href="{links[view]}">{view}</a>'
    
    return (f"{view}")
    raise ValueError(f"Unknown type: {view}")

def split_generics(args: str) -> list[str]:
    """Split generic arguments considering nested generics."""
    depth = 0
    result: list[str] = []
    current: list[str] = []
    for char in args:
        if char == ',' and depth == 0:
            result.append(''.join(current).strip())
            current = []
        else:
            if char == '<':
                depth += 1
            elif char == '>':
                depth -= 1
            current.append(char)
    if current:
        result.append(''.join(current).strip())
    return result


def proccess_rawdesc(text: str) -> str:
    i: int = 0
    length: int = len(text)
    output: List[str] = []
    ignoring_block: bool = False
    ignoring_line: bool = False
    next_line_ignore: bool = False

    while i < length:
        if ignoring_block:
            if starts_comment(text, i):
                ctext, cfull, end_index = read_comment(text, i)
                i = end_index
                if '@mkdocs-ignore-end' in ctext:
                    ignoring_block = False
            else:
                i += 1
            continue

        if ignoring_line:
            if text[i] == '\n':
                ignoring_line = False
            i += 1
            continue

        if next_line_ignore:
            if text[i] == '\n':
                next_line_ignore = False
                ignoring_line = True
                i += 1
                continue
            # If we haven't hit the newline that ends the current line yet, we still output
            # characters from the current line (the line that had the @mkdocs-ignore-next-line).
            output.append(text[i])
            i += 1
            continue

        if starts_comment(text, i):
            ctext, cfull, end_index = read_comment(text, i)
            i = end_index
            stripped = ctext.strip()
            if '@mkdocs-ignore-start' in stripped and '@mkdocs-ignore-end' in stripped:
                pass
            elif '@mkdocs-ignore-start' in stripped:
                ignoring_block = True
            elif '@mkdocs-ignore-next-line' in stripped:
                next_line_ignore = True
            elif '@mkdocs-include' in stripped:
                included = remove_directive(stripped, '@mkdocs-include')
                output.append(included)
            else:
                output.append(cfull)
        else:
            output.append(text[i])
            i += 1

    return ''.join(output)

def starts_comment(text: str, pos: int) -> bool:
    return pos+3 < len(text) and text[pos:pos+4] == '<!--'

def read_comment(text: str, start_index: int) -> Tuple[str, str, int]:
    i: int = start_index + 4
    length: int = len(text)
    comment_chars: List[str] = []
    while i < length:
        if i+2 < length and text[i:i+3] == '-->':
            full_comment = text[start_index:i+3]
            return ''.join(comment_chars), full_comment, i+3
        comment_chars.append(text[i])
        i += 1
    # malformed comment
    full_comment = text[start_index:i]
    return ''.join(comment_chars), full_comment, i

def remove_directive(text: str, directive: str) -> str:
    idx = text.find(directive)
    if idx == -1:
        return text
    before = text[:idx]
    after = text[idx+len(directive):]
    return (before + after).strip()


def parse_fn(name: str, fn: Extends) -> Tuple[str, str]:
    assert(isinstance(fn, ExtendsFunction))
    params: List[str] = []
    paramDescriptions: List[str] = []

    for arg in fn.args:
        if not arg.name:
            if arg.type == "...":
                params.append(f"...{arg.view}")
            else:
                raise ValueError(f"Failed to parse")
            continue

        # don't document self
        if arg.name == "self":
            continue

        params.append(f"{arg.name}: {arg.view}")
        desc: str = f'* ### `{arg.name}` {{: #{name}({arg.name}) }} \n (<code>{process_view(arg.view)}</code>)'
        if arg.rawdesc:
            desc += f" — {proccess_rawdesc(arg.rawdesc)}"
        desc += "\n {: .hitokage-param .md-typeset }" # https://python-markdown.github.io/extensions/attr_list/ 
        paramDescriptions.append(desc)

    fnsig: str = f"```lua\nfunction {name}({", ".join(params)}) -> {"|".join([rv.view for rv in fn.returns]) if fn.returns else "nil"}\n```"
    res: str = ""

    if len(paramDescriptions) > 0:
        res += f"\n**Parameters:**\n\n{"\n".join(paramDescriptions)}\n"

    if fn.returns and len(fn.returns) > 0:
        res += f"\n**Returns:**\n\n{"\n".join([f"* <code>{process_view(rv.view)}</code>{f' — {proccess_rawdesc(rv.rawdesc)}' if rv.rawdesc else ''}" for rv in fn.returns])}\n"

    return (fnsig, res)


# returns a link to new page
def process_fields(flds: List[FieldDefinition], filepath: str, parent: str) -> str:
    fnLinks: list[str] = []
    fnContents: list[str] = []
    moduleLinks: list[str] = []
    methodLinks: list[str] = []
    methodContents: list[str] = []
    fieldLinks: list[str] = []
    fieldContents: list[str] = []
    dotPath = ".".join(filepath.split('/')[1:])

    for fld in flds:
        titleAttr = f"title='{dotPath}.{fld.name}'"

        if not fld.visible or fld.visible != "public":
            continue

        if fld.type == "setmethod" or (isinstance(fld.extends, ExtendsFunction) and any(arg.name == "self" for arg in fld.extends.args)):
            assert(fld.name)
            methodLinks.append(f"* [{fld.name}](#method-{fld.name}){{: {titleAttr}}}")

            (fnsig, other) = parse_fn(fld.name, fld.extends)

            methodContents.append(f"""## <code class="hitokage-method">method</code> {fld.name}

{fnsig}
<div class="hitokage-content" markdown>
{proccess_rawdesc(fld.rawdesc) if fld.rawdesc else ""}

{other}                        
</div>
""")

            continue

        if fld.type == "setfield" and fld.view == "function":
            assert(fld.name)
            fnLinks.append(f"* [{fld.name}](#function-{fld.name}){{: {titleAttr}}}")

            (fnsig, other) = parse_fn(fld.name, fld.extends)

            fnContents.append(f"""## <code class="hitokage-function">function</code> {fld.name}

{fnsig}
<div class="hitokage-content" markdown>
{proccess_rawdesc(fld.rawdesc) if fld.rawdesc else ""}

{other}                        
</div>
""")
            continue

        if fld.type == "setfield" and (not fld.view in LUA_NATIVE_TYPES) and (not "table<" in fld.view):
            moduleLinks.append(f"* [{fld.name}]({fld.view}/index.html){{: {titleAttr}}}")

            continue

        if fld.type == "doc.field":
            assert(fld.name)
            fieldLinks.append(f"* [{fld.name}](#attr-{fld.name}){{: {titleAttr}}}")

            fieldContents.append(f"""## <code class="hitokage-attr">attr</code> {fld.name}

```lua
{fld.name}: {fld.view}
```
<div class="hitokage-content" markdown>
**Type**: {process_view(fld.view)}

{proccess_rawdesc(fld.rawdesc) if fld.rawdesc else ""}      
</div>
""")

            continue

        #raise ValueError(f"unsupported parse! {fld.name} {fld.type} {fld.view}")
        

    res: str = ""

    if len(moduleLinks) > 0:
        res += f"**Modules:**\n\n{"\n".join(moduleLinks)}\n\n"

    if len(fnLinks) > 0:
        res += f"**Functions:**\n\n{"\n".join(fnLinks)}\n\n"

    if len(fieldLinks) > 0:
        res += f"**Attributes:**\n\n{"\n".join(fieldLinks)}\n\n"

    if len(methodLinks) > 0:
        res += f"**Methods:**\n\n{"\n".join(methodLinks)}\n\n"
        
    if len(fnContents) > 0:
        res += "\n".join(fnContents)

    if len(fieldContents) > 0:
        res += "\n".join(fieldContents)

    if len(methodContents) > 0:
        res += "\n".join(methodContents)

    return res


SUPPORTED_TYPES = Union[Literal['mod'], Literal['userdata'], Literal['type'], Literal['alias']]
# weight, keys, markdown location
to_add_nav: List[Tuple[int, List[str], str]] = []

# matcher, transformer, filepath, type 
transformers: dict[str, Tuple[Optional[Callable[[RootItem], bool]], Callable[[RootItem, str], str], str, str]] = {}

TYPE_TO_WEIGHT: dict[SUPPORTED_TYPES, int] = {
    "mod": -100,
    "userdata": -50,
    "alias": -25,
    "type": 0
}


def normalize_link(raw_link: str) -> str:
    # Remove '[FORIEGN]file:///'
    file_url = re.sub(r"(\[FORIEGN\]|file:///|{})", "", raw_link)

    # URL-decode the path (e.g., c%3A to c:)
    file_path = urllib.parse.unquote(file_url)

    # Remove any leading slashes
    file_path = file_path.lstrip("/")

    found = None
    while (found := re.search(f"/?{REPO_NAME}/?(?![.-])", file_path)) != None:
        file_path = file_path[found.end():]

    return file_path.lstrip("/")


def create_transformer(name: str, matcher: Optional[Callable[[RootItem], bool]], type: SUPPORTED_TYPES, filepath: str) -> None:
    register_link(name, filepath)

    def default_transformer(item: RootItem, filepath: str) -> str:
        matched_dfn: DefineDefinition | None = next(
            filter(lambda x: x.view == item.name, item.defines), None
        )

        if not matched_dfn:
            matched_dfn = next(filter(lambda x: x.type == "doc.alias", item.defines), None)

        if not matched_dfn:
            raise ValueError(f"Failed to get definition of {item.name}")

        names =  filepath.split("/")[1:]
        to_add_nav.append((TYPE_TO_WEIGHT[type], names, f"{filepath[4:]}/index.md" ))

        # extract out edit link
        raw_link = matched_dfn.file

        if (raw_link):
            fixed = normalize_link(raw_link)
            print(fixed)
            mkdocs_gen_files.set_edit_path(f"{transformers[item.name][2]}/index.md", f"{REPO_URL}/tree/master/{fixed}")  # type: ignore

        # if not matched_dfn.rawdesc:
        #     print(f"Failed to get definition.rawdesc of {item.name}")
        #     return ""

        result = f"""---
title: {item.name} | API
---

# <code class="hitokage-{type}"></code> {item.name}\n

{proccess_rawdesc(matched_dfn.rawdesc) if matched_dfn.rawdesc else ""}

"""
        if item.fields and len(item.fields) > 0:
            result += process_fields(item.fields, filepath, item.name)
        elif matched_dfn.type == "doc.alias":
            result += f"""```lua
{matched_dfn.view}
```
"""

        # add step to transform type links to actual positions

        return result

    transformers[name] = (matcher, default_transformer, filepath, type)

default_matcher: Callable[[RootItem], bool] = lambda x: x.type == "type"

create_transformer("Align", default_matcher, "alias", "api/Align")
create_transformer("Component", default_matcher, "alias", "api/Component")
create_transformer("ComponentProps", default_matcher, "alias", "api/ComponentProps")
# create_transformer("BarPosition", default_matcher, "type", "api/BarProps")

create_transformer("MemoryInfo", default_matcher, "type", "api/MemoryInfo")
create_transformer("CpuLoadInfo", default_matcher, "type", "api/CpuLoadInfo")
create_transformer("BatteryInfo", default_matcher, "type", "api/BatteryInfo")
create_transformer("WeatherForecast", default_matcher, "type", "api/WeatherForecast")

create_transformer("hitokage", default_matcher, "mod", "api/hitokage")
create_transformer("bar", default_matcher, "mod", "api/hitokage/bar")
create_transformer("monitor", default_matcher, "mod", "api/hitokage/monitor")
create_transformer("unstable", default_matcher, "mod", "api/hitokage/unstable")
create_transformer("reactive", default_matcher, "mod", "api/hitokage/unstable/reactive")
create_transformer("Monitor", default_matcher, "userdata", "api/Monitor")
create_transformer("ReactiveString", default_matcher, "userdata", "api/ReactiveString")
create_transformer("MonitorGeometry", default_matcher, "type", "api/MonitorGeometry")
create_transformer("BarProps", default_matcher, "type", "api/BarProps")
create_transformer("BarOffset", default_matcher, "type", "api/BarOffset")
create_transformer("WrapBatteryProps", default_matcher, "type", "api/WrapBatteryProps")
create_transformer("WrapBoxProps", default_matcher, "type", "api/WrapBoxProps")
create_transformer("WrapClockProps", default_matcher, "type", "api/WrapClockProps")
create_transformer("WrapCpuProps", default_matcher, "type", "api/WrapCpuProps")
create_transformer("WrapIconProps", default_matcher, "type", "api/WrapIconProps")
create_transformer("WrapLabelProps", default_matcher, "type", "api/WrapLabelProps")
create_transformer("WrapMemoryProps", default_matcher, "type", "api/WrapMemoryProps")
create_transformer("WrapWeatherProps", default_matcher, "type", "api/WrapWeatherProps")
create_transformer("WrapWorkspaceProps", default_matcher, "type", "api/WrapWorkspaceProps")
create_transformer("BatteryProps", default_matcher, "type", "api/WrapBatteryProps/BatteryProps")
create_transformer("BoxProps", default_matcher, "type", "api/WrapBoxProps/BoxProps")
create_transformer("ClockProps", default_matcher, "type", "api/WrapClockProps/ClockProps")
create_transformer("CpuProps", default_matcher, "type", "api/WrapCpuProps/CpuProps")
create_transformer("IconProps", default_matcher, "type", "api/WrapIconProps/IconProps")
create_transformer("LabelProps", default_matcher, "type", "api/WrapLabelProps/LabelProps")
create_transformer("MemoryProps", default_matcher, "type", "api/WrapMemoryProps/MemoryProps")
create_transformer("WeatherProps", default_matcher, "type", "api/WrapWeatherProps/WeatherProps")
create_transformer("WorkspaceProps", default_matcher, "type", "api/WrapWorkspaceProps/WorkspaceProps")
create_transformer("Battery", default_matcher, "userdata", "api/Battery")
create_transformer("Box", default_matcher, "userdata", "api/Box")
create_transformer("Clock", default_matcher, "userdata", "api/Clock")
create_transformer("Cpu", default_matcher, "userdata", "api/Cpu")
create_transformer("Icon", default_matcher, "userdata", "api/Icon")
create_transformer("Label", default_matcher, "userdata", "api/Label")
create_transformer("Memory", default_matcher, "userdata", "api/Memory")
create_transformer("Weather", default_matcher, "userdata", "api/Weather")
create_transformer("Workspace", default_matcher, "userdata", "api/Workspace")
create_transformer("Bar", default_matcher, "userdata", "api/Bar")
create_transformer("MonitorScaleFactor", default_matcher, "type", "api/MonitorScaleFactor")
create_transformer("KomorebiAnimationStyle", default_matcher, "type", "api/komorebi/KomorebiAnimationStyle")
create_transformer("KomorebiApplicationIdentifier", default_matcher, "alias", "api/komorebi/KomorebiApplicationIdentifier")
create_transformer("KomorebiAxis", default_matcher, "alias", "api/komorebi/KomorebiAxis")
create_transformer("KomorebiBorderImplementation", default_matcher, "alias", "api/komorebi/KomorebiBorderImplementation")
create_transformer("KomorebiBorderStyle", default_matcher, "alias", "api/komorebi/KomorebiBorderStyle")
create_transformer("KomorebiColumn", default_matcher, "type", "api/komorebi/KomorebiColumn")
create_transformer("KomorebiColumnSplit", default_matcher, "alias", "api/komorebi/KomorebiColumnSplit")
create_transformer("KomorebiColumnSplitWithCapacity", default_matcher, "alias", "api/komorebi/KomorebiColumnSplitWithCapacity")
create_transformer("KomorebiColumnWidth", default_matcher, "type", "api/komorebi/KomorebiColumnWidth")
create_transformer("KomorebiContainer", default_matcher, "type", "api/komorebi/KomorebiContainer")
create_transformer("KomorebiCustomLayout", default_matcher, "alias", "api/komorebi/KomorebiCustomLayout")
create_transformer("KomorebiCycleDirection", default_matcher, "alias", "api/komorebi/KomorebiCycleDirection")
create_transformer("KomorebiDefaultLayout", default_matcher, "alias", "api/komorebi/KomorebiDefaultLayout")
create_transformer("KomorebiFocusFollowsMouseImplementation", default_matcher, "alias", "api/komorebi/KomorebiFocusFollowsMouseImplementation")
create_transformer("KomorebiLayout", default_matcher, "type", "api/komorebi/KomorebiLayout")
create_transformer("KomorebiMonitor", default_matcher, "type", "api/komorebi/KomorebiMonitor")
create_transformer("KomorebiMonitorRing", default_matcher, "type", "api/komorebi/KomorebiMonitorRing")
create_transformer("KomorebiMoveBehaviour", default_matcher, "alias", "api/komorebi/KomorebiMoveBehaviour")
create_transformer("KomorebiNotification", default_matcher, "type", "api/komorebi/KomorebiNotification")
create_transformer("KomorebiNotificationEvent", default_matcher, "type", "api/komorebi/KomorebiNotificationEvent")
create_transformer("KomorebiOperationBehaviour", default_matcher, "alias", "api/komorebi/KomorebiOperationBehaviour")
create_transformer("KomorebiRect", default_matcher, "type", "api/komorebi/KomorebiRect")
create_transformer("KomorebiSizing", default_matcher, "alias", "api/komorebi/KomorebiSizing")
create_transformer("KomorebiSocketMessage", default_matcher, "type", "api/komorebi/KomorebiSocketMessage")
create_transformer("KomorebiStackbarLabel", default_matcher, "alias", "api/komorebi/KomorebiStackbarLabel")
create_transformer("KomorebiStackbarMode", default_matcher, "alias", "api/komorebi/KomorebiStackbarMode")
create_transformer("KomorebiState", default_matcher, "type", "api/komorebi/KomorebiState")
create_transformer("KomorebiWindow", default_matcher, "type", "api/komorebi/KomorebiWindow")
create_transformer("KomorebiWindowContainerBehaviour", default_matcher, "alias", "api/komorebi/KomorebiWindowContainerBehaviour")
create_transformer("KomorebiWindowManagerEvent", default_matcher, "alias", "api/komorebi/KomorebiWindowManagerEvent")
create_transformer("KomorebiWindowRing", default_matcher, "type", "api/komorebi/KomorebiWindowRing")
create_transformer("KomorebiWinEvent", default_matcher, "alias", "api/komorebi/KomorebiWinEvent")
create_transformer("KomorebiWorkspace", default_matcher, "type", "api/komorebi/KomorebiWorkspace")
create_transformer("KomorebiWorkspaceRing", default_matcher, "type", "api/komorebi/KomorebiWorkspaceRing")
create_transformer("Base", default_matcher, "userdata", "api/Base")
create_transformer("BaseProps", default_matcher, "type", "api/BaseProps")

for item in data.root:
    if item.name in transformers and (
        transformers[item.name][0](item) if transformers[item.name][0] else True
    ):
        print(f"Found match for {item.name}")
        transfomed = transformers[item.name][1](item, transformers[item.name][2])
        print(transfomed)
        with mkdocs_open(
            f"{transformers[item.name][2]}/index.md", "a", encoding="utf-8"
        ) as f:
            f.writelines(transfomed)
    else:
        print(f"Failed to find matcher for {item.name}")

def prepend_code_to_nav(x: re.Match[str]) -> str:
    name: str = x.group(1)

    return f"* [<code class='hitokage-{transformers[name][3]}'></code> {name}]"
    
with mkdocs_gen_files.open("api/SUMMARY.md", "w") as nav_file:
    nav = mkdocs_gen_files.Nav()
    fixed_nav = sorted(to_add_nav, key=lambda x: ("Komorebi" in x[2], x[0], x[2]))
    for (_, path, md) in fixed_nav:
        nav[*path] = md
    res = "".join(nav.build_literate_nav())
    res = re.sub(f"\\* \\[({'|'.join(transformers.keys())})\\]", prepend_code_to_nav, res, flags=re.MULTILINE)
    print(res)
    nav_file.writelines(f"{res}")

print(links)


# IGNORE = (
#     "any",
#     "arg",
#     "assert",
#     "boolean",
#     "collectgarbage",
#     "coroutine",
#     "coroutinelib",
#     "debug",
#     "debuglib",
#     "debuginfo",
#     "dofile",
#     "error",
#     "exitcode",
#     "false",
#     "file*",
#     "filetype",
#     "function",
#     "gcoptions",
#     "getfenv",
#     "getmetatable",
#     "hookmask",
#     "infowhat",
#     "integer",
#     "io",
#     "iolib",
#     "ipairs",
#     "lightuserdata",
#     "load",
#     "loadfile",
#     "loadmode",
#     "loadstring",
#     "localecategory",
#     "math",
#     "mathlib",
#     "metatable",
#     "module",
#     "newproxy",
#     "next",
#     "nil",
#     "number",
#     "openmode",
#     "os",
#     "osdate",
#     "osdateparam",
#     "os",
#     "oslib",
#     "package",
#     "packagelib",
#     "pairs",
#     "pcall",
#     "popenmode",
#     "print",
#     "rawequal",
#     "rawget",
#     "rawlen",
#     "rawset",
#     "readmode",
#     "require",
#     "seekwhence",
#     "select",
#     "setfenv",
#     "setmetatable",
#     "string",
#     "stringlib",
#     "table",
#     "tablelib",
#     "thread",
#     "tonumber",
#     "tostring",
#     "true",
#     "type",
#     "unknown",
#     "unpack",
#     "userdata",
#     "utf8",
#     "utf8lib",
#     "vbuf",
#     "warn",
#     "xpcall",
#     )
