--- @meta hitokage.widgets.bar

--- @class BarOffset
--- @field x integer?
--- @field y integer?

-------------------------------------------------------------------------------
--- @class BarProps : BaseProps, HasChildrenProps
--- @field width integer?
--- @field height integer?
--- @field offset BarOffset?
---
--- Whether the children should all be the same size. Defaults to `true`
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Box.homogeneous.html)
---
--- @field homogeneous boolean?

-------------------------------------------------------------------------------
--- Links to BarUserData in 'hitokage-lua\src\widgets\bar.rs'
--- @class Bar : Base, HasChildren
---
--- Wrapper around bar:is_ready()
--- @field ready boolean
---
--- Wrapper around bar:get_geometry()
--- @field geometry boolean
local bar_instance = {}

--- @alias BarArray table<number, Bar>

--- Checks if the bar has been instantiated in gtk4
--- @return boolean
function bar_instance:is_ready() end

--- Get the geometry of the bar
--- @return MonitorGeometry
function bar_instance:get_geometry() end
