---@meta hitokage.components.bar

---@class BarOffset
---@field x integer?
---@field y integer?

-------------------------------------------------------------------------------
---@class BarProps : BoxProps
---@field width integer?
---@field height integer?
---@field offset BarOffset?

-------------------------------------------------------------------------------
---Links to BarUserData in 'hitokage-lua\src\components\bar.rs'
---@class Bar : Base, Box
---
---Wrapper around [`is_ready`](lua://Bar.is_ready)
---@field ready boolean
---
---Wrapper around [`get_geometry`](lua://Bar.get_geometry)
---@field geometry MonitorGeometry
local bar_instance = {}

---@alias BarArray table<number, Bar>

---Checks if the bar has been instantiated in gtk4
---@return boolean
function bar_instance:is_ready() end

---Get the geometry of the bar
---@return MonitorGeometry
function bar_instance:get_geometry() end
