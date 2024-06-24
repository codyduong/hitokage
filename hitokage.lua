--- @meta hitokage

--- This is the global module for [hitokage](https://github.com/codyduong/hitokage)
---
--- [View documentation](https://codyduong.dev/hitokage/lua/hitokage)
---
--- @class hitokagelib
_G.hitokage = {}
_G._not_deadlocked = function() end
_G._subscribers = {}
_G._subscriptions = {}

-------------------------------------------------------------------------------
--- generic global functions

--- @vararg any
function hitokage.debug(...) end
--- @vararg any
function hitokage.error(...) end
--- @vararg any
function hitokage.info(...) end

--- Sleep function in milliseconds
--- @param ms number Amount of time to sleep.
function hitokage.sleep_ms(ms) end

-------------------------------------------------------------------------------
--- Represents the bar module
--- @class bar
local bar = {}

--- Creates a new bar
--- @param monitor Monitor
--- @param bar_props BarProps
--- @return BarInstance
function bar.create(monitor, bar_props) end

--- Links to WorkspaceProps in 'hitokage-core\src\widgets\workspace.rs'
--- @class WorkspaceProps

--- Links to ClockProps in 'hitokage-core\src\widgets\clock.rs'
--- @class ClockProps
--- @fields format string?

--- Links to BoxProps in 'hitokage-core\src\widgets\box.rs'
--- @class BoxProps

--- Links to WidgetProps in 'hitokage-core\src\widgets\mod.rs'
--- @class WidgetProps
--- @field Workspace table? Optional workspace configuration
--- @field Clock ClockProps? Optional clock configuration
--- @field Box table? Optional box configuration

--- Array of WidgetProps
--- @alias WidgetPropsVec table<number, WidgetProps>

--- @class BarProps
--- @field widgets WidgetPropsVec

--- Links to BarInstanceUserData in 'hitokage-lua\src\widgets\bar.rs'
--- @class BarInstance
--- Wrapper around bar:is_ready()
--- @field ready boolean
--- Wrapper around bar:get_widgets()
--- @field widgets any -- @codyduong TODO fix this return type
--- Wrapper around bar:get_geometry()
--- @field geometry boolean
local bar_instance = {}

--- @alias BarInstanceArray table<number, BarInstance>

--- Gets the bar id
--- @return number
function bar_instance:get_id() end

--- Checks if the bar has been instantiated in gtk4
--- @return boolean
function bar_instance:is_ready() end

--- Gets the widgets on the bar
--- @return boolean -- @codyduong TODO fix this return type
function bar_instance:get_widgets() end

--- Gets the geometry of the bar
--- @return MonitorGeometry
function bar_instance:get_geometry() end

-------------------------------------------------------------------------------
--- Represents the monitor module
--- @class monitor
local monitor = {}

--- @return MonitorVec
function monitor.get_all() end

--- @return Monitor
function monitor.get_primary() end

--- Links to 'pub struct MonitorGeometry' in 'hitokage-core\src\lua\monitor.rs'
--- @class MonitorGeometry
--- @field x number
--- @field y number
--- @field width number
--- @field height number

--- Links to 'pub struct Monitor' in 'hitokage-core\src\lua\monitor.rs'
--- @class Monitor
--- @field connecter string | nil,
--- @field description string | nil,
--- @field geometry MonitorGeometry,
--- @field manufacturer string | nil,
--- @field model string | nil,
--- This value is in millihertz (mHz) not hertz (Hz)
--- @field refresh_rate number,
--- @field is_primary boolean,
--- @field device string,
--- @field device_id string,
--- @field id number,
--- @field name string,
--- @field scale_factor MonitorScaleFactor,

--- @alias MonitorVec table<number, Monitor>

--- Links to 'pub struct MonitorScaleFactor' in 'hitokage-core\src\lua\monitor.rs'
--- @class MonitorScaleFactor
--- @field x number,
--- @field y number,

-------------------------------------------------------------------------------
--- Compose all the modules
hitokage.bar = bar
hitokage.monitor = monitor