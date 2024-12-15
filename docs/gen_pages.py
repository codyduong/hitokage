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


def proccess_rawdesc(md_content: str) -> str:
    """
    Transforms Markdown content by processing specific @mkdocs-* tags in HTML comments.

    Supported tags:
      - @mkdocs-remove-next-line
      - @mkdocs-ignore-start
      - @mkdocs-ignore-end
      - @mkdocs-include

    Args:
        md_content (str): The input Markdown content as a string.

    Returns:
        str: The transformed Markdown content as a string.

    Raises:
        ValueError: If an unsupported @mkdocs-* tag is encountered.
    """
    lines: List[str] = md_content.splitlines()
    result: List[str] = []
    skip_block = False

    i = 0
    while i < len(lines):
        line = lines[i]

        if "<!--@mkdocs-ignore-next-line-->" in line:
            i += 1  # Skip the next line
            i += 1
            continue

        if "<!--@mkdocs-ignore-start-->" in line:
            skip_block = True
            i += 1
            continue

        if "<!--@mkdocs-ignore-end-->" in line:
            skip_block = False
            i += 1
            continue

        if "<!--@mkdocs-include" in line:
            # Extract the content to include, stripping tags
            include_content: List[str] = []
            while i < len(lines) and not lines[i].strip().endswith("-->"):
                include_content.append(lines[i])
                i += 1

            # Append the final line of the include block
            if i < len(lines):
                include_content.append(lines[i])

            include_content_str = "\n".join(include_content)
            include_match = re.search(r"<!--@mkdocs-include\s+(.*?)\s+-->", include_content_str, re.DOTALL)
            if include_match:
                result.append(include_match.group(1).strip())
            else:
                raise ValueError(f"Malformed @mkdocs-include tag near line {i + 1}.")
            i += 1
            continue

        if "<!--@mkdocs-" in line:
            # Detect unsupported tags
            if not any(tag in line for tag in [
                "<!--@mkdocs-ignore-next-line-->",
                "<!--@mkdocs-ignore-start-->",
                "<!--@mkdocs-ignore-end-->",
                "<!--@mkdocs-include"]):
                raise ValueError(f"Unsupported @mkdocs-* tag encountered at line {i + 1}: {line}")

        if not skip_block:
            result.append(line)

        i += 1

    return "\n".join(result)


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

        params.append(f"{arg.name}: {arg.view}")
        desc: str = f"* `{arg.name}` ({html.escape(arg.view)})"
        if arg.rawdesc:
            desc += f"— {arg.rawdesc}"
        paramDescriptions.append(desc)

    fnsig: str = f"```lua\nfunction {name}({", ".join(params)}) -> {"|".join([rv.view for rv in fn.returns]) if fn.returns else "nil"}\n```"
    res: str = ""

    if len(paramDescriptions) > 0:
        res += f"\n**Parameters:**\n\n{"\n".join(paramDescriptions)}\n"

    if fn.returns and len(fn.returns) > 0:
        res += f"\n**Returns:**\n\n{"\n".join([f"* `{rv.view}`{f'— {rv.rawdesc}' if rv.rawdesc else ''}" for rv in fn.returns])}\n"

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

    for fld in flds:
        if not fld.visible or fld.visible != "public":
            continue

        if fld.type == "setfield" and fld.view == "function":
            assert(fld.name)
            fnLinks.append(f"* [{fld.name}](#function-{fld.name})")

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
            moduleLinks.append(f"* [{fld.name}]({fld.view}/index.html)")

            continue

        if fld.type == "setmethod":
            assert(fld.name)
            methodLinks.append(f"* [{fld.name}](#method-{fld.name})")

            (fnsig, other) = parse_fn(fld.name, fld.extends)

            methodContents.append(f"""## <code class="hitokage-method">method</code> {fld.name}

{fnsig}
<div class="hitokage-content" markdown>
{proccess_rawdesc(fld.rawdesc) if fld.rawdesc else ""}

{other}                        
</div>
""")

            continue

        if fld.type == "doc.field":
            assert(fld.name)
            fieldLinks.append(f"* [{fld.name}](#attr-{fld.name})")

            fieldContents.append(f"""## <code class="hitokage-attr">attr</code> {fld.name}

```lua
{fld.name}: {fld.view}
```
<div class="hitokage-content" markdown>
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


SUPPORTED_TYPES = Union[Literal['mod'], Literal['userdata'], Literal['type']]
# weight, keys, markdown location
to_add_nav: List[Tuple[int, List[str], str]] = []

# matcher, transformer, filepath, type 
transformers: dict[str, Tuple[Optional[Callable[[RootItem], bool]], Callable[[RootItem, str], str], str, str]] = {}

TYPE_TO_WEIGHT: dict[SUPPORTED_TYPES, int] = {
    "mod": -100,
    "userdata": -50,
    "type": 0
}


def normalize_link(raw_link: str) -> str:
    # Remove '[FORIEGN]file:///'
    file_url = re.sub(r"(\[FORIEGN\]|file:///)", "", raw_link)

    # URL-decode the path (e.g., c%3A to c:)
    file_path = urllib.parse.unquote(file_url)

    # Remove any leading slashes
    file_path = file_path.lstrip("/")

    workspace_index = file_path.find(REPO_NAME)
    if workspace_index != -1:
        # Truncate the path to start from the workspace directory
        return file_path[workspace_index+len(REPO_NAME)+1:]
    else:
        raise ValueError(f"Failed to parse url: {raw_link}\n")


def create_transformer(name: str, matcher: Optional[Callable[[RootItem], bool]], type: SUPPORTED_TYPES, filepath: str) -> None:
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
create_transformer("WidgetBatteryProps", default_matcher, "type", "api/WidgetBatteryProps")
create_transformer("WidgetBoxProps", default_matcher, "type", "api/WidgetBoxProps")
create_transformer("WidgetClockProps", default_matcher, "type", "api/WidgetClockProps")
create_transformer("WidgetCpuProps", default_matcher, "type", "api/WidgetCpuProps")
create_transformer("WidgetIconProps", default_matcher, "type", "api/WidgetIconProps")
create_transformer("WidgetLabelProps", default_matcher, "type", "api/WidgetLabelProps")
create_transformer("WidgetMemoryProps", default_matcher, "type", "api/WidgetMemoryProps")
create_transformer("WidgetWeatherProps", default_matcher, "type", "api/WidgetWeatherProps")
create_transformer("WidgetWorkspaceProps", default_matcher, "type", "api/WidgetWorkspaceProps")
create_transformer("BatteryProps", default_matcher, "type", "api/WidgetBatteryProps/BatteryProps")
create_transformer("BoxProps", default_matcher, "type", "api/WidgetBoxProps/BoxProps")
create_transformer("ClockProps", default_matcher, "type", "api/WidgetClockProps/ClockProps")
create_transformer("CpuProps", default_matcher, "type", "api/WidgetCpuProps/CpuProps")
create_transformer("IconProps", default_matcher, "type", "api/WidgetIconProps/IconProps")
create_transformer("LabelProps", default_matcher, "type", "api/WidgetLabelProps/LabelProps")
create_transformer("MemoryProps", default_matcher, "type", "api/WidgetMemoryProps/MemoryProps")
create_transformer("WeatherProps", default_matcher, "type", "api/WidgetWeatherProps/WeatherProps")
create_transformer("WorkspaceProps", default_matcher, "type", "api/WidgetWorkspaceProps/WorkspaceProps")
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
create_transformer("Align", default_matcher, "type", "api/Align")
create_transformer("KomorebiAnimationStyle", default_matcher, "type", "api/komorebi/KomorebiAnimationStyle")
create_transformer("KomorebiApplicationIdentifier", default_matcher, "type", "api/komorebi/KomorebiApplicationIdentifier")
create_transformer("KomorebiAxis", default_matcher, "type", "api/komorebi/KomorebiAxis")
create_transformer("KomorebiBorderImplementation", default_matcher, "type", "api/komorebi/KomorebiBorderImplementation")
create_transformer("KomorebiBorderStyle", default_matcher, "type", "api/komorebi/KomorebiBorderStyle")
create_transformer("KomorebiColumn", default_matcher, "type", "api/komorebi/KomorebiColumn")
create_transformer("KomorebiColumnSplit", default_matcher, "type", "api/komorebi/KomorebiColumnSplit")
create_transformer("KomorebiColumnSplitWithCapacity", default_matcher, "type", "api/komorebi/KomorebiColumnSplitWithCapacity")
create_transformer("KomorebiColumnWidth", default_matcher, "type", "api/komorebi/KomorebiColumnWidth")
create_transformer("KomorebiContainer", default_matcher, "type", "api/komorebi/KomorebiContainer")
create_transformer("KomorebiCustomLayout", default_matcher, "type", "api/komorebi/KomorebiCustomLayout")
create_transformer("KomorebiCycleDirection", default_matcher, "type", "api/komorebi/KomorebiCycleDirection")
create_transformer("KomorebiDefaultLayout", default_matcher, "type", "api/komorebi/KomorebiDefaultLayout")
create_transformer("KomorebiFocusFollowsMouseImplementation", default_matcher, "type", "api/komorebi/KomorebiFocusFollowsMouseImplementation")
create_transformer("KomorebiLayout", default_matcher, "type", "api/komorebi/KomorebiLayout")
create_transformer("KomorebiMonitor", default_matcher, "type", "api/komorebi/KomorebiMonitor")
create_transformer("KomorebiMonitorRing", default_matcher, "type", "api/komorebi/KomorebiMonitorRing")
create_transformer("KomorebiMoveBehaviour", default_matcher, "type", "api/komorebi/KomorebiMoveBehaviour")
create_transformer("KomorebiNotification", default_matcher, "type", "api/komorebi/KomorebiNotification")
create_transformer("KomorebiNotificationEvent", default_matcher, "type", "api/komorebi/KomorebiNotificationEvent")
create_transformer("KomorebiOperationBehaviour", default_matcher, "type", "api/komorebi/KomorebiOperationBehaviour")
create_transformer("KomorebiRect", default_matcher, "type", "api/komorebi/KomorebiRect")
create_transformer("KomorebiSizing", default_matcher, "type", "api/komorebi/KomorebiSizing")
create_transformer("KomorebiSocketMessage", default_matcher, "type", "api/komorebi/KomorebiSocketMessage")
create_transformer("KomorebiStackbarLabel", default_matcher, "type", "api/komorebi/KomorebiStackbarLabel")
create_transformer("KomorebiStackbarMode", default_matcher, "type", "api/komorebi/KomorebiStackbarMode")
create_transformer("KomorebiState", default_matcher, "type", "api/komorebi/KomorebiState")
create_transformer("KomorebiWindow", default_matcher, "type", "api/komorebi/KomorebiWindow")
create_transformer("KomorebiWindowContainerBehaviour", default_matcher, "type", "api/komorebi/KomorebiWindowContainerBehaviour")
create_transformer("KomorebiWindowManagerEvent", default_matcher, "type", "api/komorebi/KomorebiWindowManagerEvent")
create_transformer("KomorebiWindowRing", default_matcher, "type", "api/komorebi/KomorebiWindowRing")
create_transformer("KomorebiWinEvent", default_matcher, "type", "api/komorebi/KomorebiWinEvent")
create_transformer("KomorebiWorkspace", default_matcher, "type", "api/komorebi/KomorebiWorkspace")
create_transformer("KomorebiWorkspaceRing", default_matcher, "type", "api/komorebi/KomorebiWorkspaceRing")


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
    fixed_nav = sorted(to_add_nav, key=lambda x: (x[0], x[2]))
    for (_, path, md) in fixed_nav:
        nav[*path] = md
    res = "".join(nav.build_literate_nav())
    res = re.sub(f"\\* \\[({'|'.join(transformers.keys())})\\]", prepend_code_to_nav, res, flags=re.MULTILINE)
    print(res)
    nav_file.writelines(f"{res}")


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
