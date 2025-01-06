---@meta hitokage.components.cpu

--------------------------------------------------------------------------------
---Links to CpuProps in 'hitokage-core\src\components\cpu.rs'

---A native component within `hitokage` that displays cpu information.
---
---See <!--@mkdocs-ignore-start-->[`ComponentProps`](lua://ComponentProps)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/ComponentProps" title="ComponentProps">`ComponentProps`</a> -->
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
---
---    ```lua
---    monitor = hitokage.monitor.get_primary()
---
---    ---@type ClockProps
---    cpu_props = { format = "{{pad "left" (concat (round (mult usage 100) 1) "%") 6}}" }
---
---    monitor:attach({
---      children = {
---        cpu = cpu_props,
---      },
---    })
---    ```
---
---The mounted API is documented here: <!--@mkdocs-ignore-start-->[`Cpu`](lua://Cpu)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/Cpu" title="Cpu">`Cpu`</a> -->
---
---@class CpuProps : BaseProps
---
---A handlebars template string or function accepts CpuLoadInfo and returns a handlebars template string
---
---@codyduong TODO explain this
---
---@field format string | ReactiveString | fun(cpu_info: CpuLoadInfo): string

--------------------------------------------------------------------------------
---Links to CpuUserData hitokage-lua\src\components\cpu.rs

---A userdata which corresponds to the mounted version of <!--@mkdocs-ignore-start-->[`CpuProps`](lua://CpuProps)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/WrapCpuProps/CpuProps" title="CpuProps">`CpuProps`</a> -->
---
---> A native component within `hitokage` that displays cpu information.
---
---This userdata can be retrieved using:
---<!--@mkdocs-ignore-start-->
---* [`Box:get_child_by_id`](lua://Box.get_child_by_id)
---* [`Box:get_children`](lua://Box.get_children)
---* [`Bar:get_child_by_id`](lua://Bar.get_child_by_id)
---* [`Bar:get_children`](lua://Bar.get_children)
---<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include * <a href="/hitokage/api/Box#method-get_child_by_id" title="Box#method-get_child_by_id">`Box:get_child_by_id`</a>
---* <a href="/hitokage/api/Box#method-get_children" title="Box#method-get_children">`Box:get_children`</a>
---* <a href="/hitokage/api/Bar#method-get_child_by_id" title="Bar#method-get_child_by_id">`Bar:get_child_by_id`</a>
---* <a href="/hitokage/api/Bar#method-get_children" title="Bar#method-get_children">`Bar:get_children`</a>
----->
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
---
---    ```lua
---    monitor = hitokage.monitor.get_primary()
---
---    ---@type CpuProps
---    cpu_props = { id = "cpu1", format = "{{pad "left" (concat (round (mult usage 100) 1) "%") 6}}" }
---
---    bar = monitor:attach({
---      children = {
---        cpu = cpu_props,
---      },
---    })
---
---    cpu1 = bar:get_child_by_id("cpu1")
---    ```
---
---@class Cpu : Base
---
---@field type 'Cpu'
local cpu_instance = {}

---Get the type of widget
---@return 'Cpu'
function cpu_instance:get_type() end

---Get the format string
---@return string
function cpu_instance:get_format() end

---Get the reactive format string
---@return ReactiveString
---@nodiscard
function cpu_instance:get_format_reactive() end

---Set the format string
---@param string string
---@return nil
function cpu_instance:set_format(string) end

--------------------------------------------------------------------------------
---Links to CpuLoadInfo hitokage-core\src\components\cpu.rs

---@class CpuLoadInfo
---
---@field cores table<number, CpuLoadCoreInfo>
---@field user number
---@field nice number
---@field system number
---@field interrupt number
---@field idle number
---@field usage number

---Links to CpuLoadCoreInfo hitokage-core\src\components\cpu.rs
---@class CpuLoadCoreInfo
---@field user number
---@field nice number
---@field system number
---@field interrupt number
---@field idle number
---@field usage number
