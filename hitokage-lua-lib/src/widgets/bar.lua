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

--- Gets the bar id
--- @return number
function bar_instance:get_id() end

--- Gets the widgets on the bar
--- @return table<number, Box | Clock | Workspace >
function bar_instance:get_widgets() end

-------------------------------------------------------------------------------
--- Links to WidgetProps in 'hitokage-core\src\widgets\mod.rs'
--- @class WidgetProps
--- @field Workspace table? Optional workspace configuration
--- @field Clock ClockProps? Optional clock configuration
--- @field Box table? Optional box configuration

--- Array of WidgetProps
--- @alias WidgetPropsVec table<number, WidgetProps>

--- @class BarOffset
--- @field x number?
--- @field y number?

-------------------------------------------------------------------------------
--- Represents the bar module
--- @class barlib
local bar = {}

--- @class BarProps
--- @field widgets WidgetPropsVec
--- @field width number?
--- @field height number?
--- @field offset BarOffset?

--- Creates a new bar
--- @param monitor Monitor
--- @param bar_props BarProps
--- @return Bar
function bar.create(monitor, bar_props) end

return bar
