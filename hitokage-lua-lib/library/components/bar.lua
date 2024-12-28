---@meta hitokage.components.bar

---@class BarOffset
---@field x integer?
---@field y integer?

--------------------------------------------------------------------------------

---@class BarProps : BoxProps
---@field width integer?
---@field height integer?
---@field offset BarOffset?

--------------------------------------------------------------------------------
---Links to BarUserData in 'hitokage-lua\src\components\bar.rs'

---A bar is created using <!--@mkdocs-ignore-start-->[`monitor:attach`](lua://Monitor.attach)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/Monitor#method-attach" title="Monitor#method-attach">`monitor:attach`</a> -->
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
