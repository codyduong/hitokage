---@meta hitokage.components.bar

---@class BarOffset
---@field x integer?
---@field y integer?

--------------------------------------------------------------------------------

---Used as entrypoint for <!--@mkdocs-ignore-start-->[`monitor:attach`](lua://Monitor.attach)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/Monitor#method-attach" title="Monitor#method-attach">`monitor:attach`</a> -->
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
---
---    ```lua
---    monitor = hitokage.monitor.get_primary()
--- 
---    ---@type BarProps
---    bar_props = {
---    	children = {
---    		 { Workspace = { halign = "Start", item_height = 24, item_width = 24 } },
---    		 { Box = {} },
---    		 { Clock = { format = "%a %b %u %r", halign = "End" } },
---    	 },
---    	 homogeneous = true,
---    }
--- 
---    monitor:attach(bar_props)
---    ```
---
---The mounted API is documented here: <!--@mkdocs-ignore-start-->[`Bar`](lua://Bar)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/Bar" title="Bar">`Bar`</a> -->
---
---@class BarProps : BoxProps
---@field width integer?
---@field height integer?
---@field offset BarOffset?

--------------------------------------------------------------------------------
---Links to BarUserData in 'hitokage-lua\src\components\bar.rs'

---A userdata which corresponds to the mounted version of <!--@mkdocs-ignore-start-->[`BarProps`](lua://BarProps)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/BarProps" title="BarProps">`BarProps`</a> -->
---
---A bar is created using <!--@mkdocs-ignore-start-->[`monitor:attach`](lua://Monitor.attach)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/Monitor#method-attach" title="Monitor#method-attach">`monitor:attach`</a> -->
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
---
---    ```lua
---    monitor = hitokage.monitor.get_primary()
--- 
---    ---@type BarProps
---    bar_props = {
---    	children = {
---    		 { Workspace = { halign = "Start", item_height = 24, item_width = 24 } },
---    		 { Box = {} },
---    		 { Clock = { format = "%a %b %u %r", halign = "End" } },
---    	 },
---    	 homogeneous = true,
---    }
--- 
---    bar_userdata = monitor:attach(bar_props)
---    ```
---
---@class Bar : Base, Box
---
---@field type 'Bar'
---
---Wrapper around <!--@mkdocs-ignore-next-line-->
---[`is_ready`](lua://Bar.is_ready)
---<!--@mkdocs-include <a href="#method-is_ready" title="bar#method-is_ready">`is_ready`</a>-->
---@field ready boolean
---
---Wrapper around <!--@mkdocs-ignore-next-line-->
---[`get_geometry`](lua://Bar.get_geometry)
---<!--@mkdocs-include <a href="#method-get_geometry" title="bar#method-get_geometry">`get_geometry`</a>-->
---@field geometry MonitorGeometry
local bar_instance = {}

---@alias BarArray table<number, Bar>

---Checks if the bar has been instantiated in gtk4
---@return boolean
function bar_instance:is_ready() end

---Get the geometry of the bar
---@return MonitorGeometry
function bar_instance:get_geometry() end
