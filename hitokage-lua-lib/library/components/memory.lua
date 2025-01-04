---@meta hitokage.components.memory

--------------------------------------------------------------------------------
---Links to MemoryProps in 'hitokage-core\src\components\memory.rs'

---A native component within `hitokage` that displays current memory information.
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
--- 	 local mem_str = 
---      '{{pad "left" (round (div used 1024) 1) 4}} ({{ pad "left" (concat (round (mult (div used total) 100) 1) "%") 4 }})'
--- 
---    ---@type MemoryProps
---    memory_props = { format = mem_str }
--- 
---    monitor:attach({
---      children = {
---        Memory = memory_props, 
---      },
---    })
---    ```
---
---The mounted API is documented here: <!--@mkdocs-ignore-start-->[`Memory`](lua://Memory)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/Memory" title="Memory">`Memory`</a> -->
---
---@class MemoryProps : BaseProps
---
---A handlebars template string or function accepts CpuLoadInfo and returns a handlebars template string
---
---@codyduong TODO explain this
---
---@field format string | ReactiveString | fun(MemoryInfo: MemoryInfo): string

--------------------------------------------------------------------------------
---Links to MemoryUserData hitokage-lua\src\components\memory.rs

---A userdata which corresponds to the mounted version of <!--@mkdocs-ignore-start-->[`MemoryProps`](lua://MemoryProps)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/WrapMemoryProps/MemoryProps" title="MemoryProps">`MemoryProps`</a> -->
---
---> A native component within `hitokage` that displays current memory information.
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
--- 	 local mem_str = 
---      '{{pad "left" (round (div used 1024) 1) 4}} ({{ pad "left" (concat (round (mult (div used total) 100) 1) "%") 4 }})'
--- 
---    ---@type BatteryProps
---    memory_props = { id = "memory1", format = mem_str }
--- 
---    bar = monitor:attach({
---      children = {
---        Memory = memory_props, 
---      },
---    })
--- 
---    memory1 = bar:get_child_by_id("memory1")
---    ```
---
---@class Memory : Base
---
---@field type 'Memory'
local memory_instance = {}

---Get the type of widget
---@return 'Memory'
function memory_instance:get_type() end

---Get the format string
---@return string
function memory_instance:get_format() end

---Get the reactive format string
---@return ReactiveString
---@nodiscard
function memory_instance:get_format_reactive() end

---Set the format string
---@param string string
---@return nil
function memory_instance:set_format(string) end

--------------------------------------------------------------------------------
---Links to MemoryInfo hitokage-core\src\components\memory.rs

---@class MemoryInfo
---
---@field free number
---@field total number
---@field used number
---@field swap_free number
---@field swap_total number
---@field swap_used number
