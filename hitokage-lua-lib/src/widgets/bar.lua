-------------------------------------------------------------------------------
--- Links to BarUserData in 'hitokage-lua\src\widgets\bar.rs'
--- @class Bar
---
--- Wrapper around bar:is_ready()
--- @field ready boolean
---
--- Wrapper around bar:get_geometry()
--- @field geometry boolean
---
--- Wrapper around bar:get_widgets()
--- @field widgets any -- @codyduong TODO fix this return type
---
local bar_instance = {}

--- @alias BarArray table<number, Bar>

--- Checks if the bar has been instantiated in gtk4
--- @return boolean
function bar_instance:is_ready() end

--- Gets the geometry of the bar
--- @return MonitorGeometry
function bar_instance:get_geometry() end

--- Gets the widgets on the bar
--- @return table<number, Box | Clock | Workspace >
function bar_instance:get_widgets() end

-------------------------------------------------------------------------------
--- Links to WidgetProps in 'hitokage-core\src\widgets\mod.rs'
--- @class WidgetWorkspaceProps
--- @field Clock ClockProps
--- @class WidgetBoxProps
--- @field Box BoxProps
--- @field Workspace WorkspaceProps
--- @class WidgetClockProps

--- Array of WidgetProps
--- @alias WidgetPropsVec table<number, WidgetBoxProps | WidgetClockProps | WidgetWorkspaceProps>

--- @class BarOffset
--- @field x integer?
--- @field y integer?

-------------------------------------------------------------------------------
--- Represents the bar module
--- @class barlib
local bar = {}

--- @class BarProps
--- @field widgets WidgetPropsVec
--- @field width integer?
--- @field height integer?
--- @field offset BarOffset?

--- Creates a new bar
--- @param monitor Monitor
--- @param bar_props BarProps
--- @return Bar
function bar.create(monitor, bar_props) end

return bar
