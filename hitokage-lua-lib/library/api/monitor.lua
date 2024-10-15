---@meta hitokage.api.monitor

-------------------------------------------------------------------------------
---Represents the monitor module
---@class monitorlib
local monitor = {}

---@return MonitorVec
function monitor.get_all() end

---@return Monitor
function monitor.get_primary() end

-------------------------------------------------------------------------------
---Links to 'pub struct MonitorGeometry' in 'hitokage-core\src\lua\monitor.rs'
---@class MonitorGeometry
---@field x number
---@field y number
---@field width number
---@field height number

-------------------------------------------------------------------------------
---Links to 'pub struct MonitorScaleFactor' in 'hitokage-core\src\lua\monitor.rs'
---@class MonitorScaleFactor
---@field x number,
---@field y number,

-------------------------------------------------------------------------------
---Links to 'pub struct Monitor' in 'hitokage-core\src\lua\monitor.rs'
---@class Monitor
---@field connector string | nil,
---@field description string | nil,
---@field geometry MonitorGeometry,
---@field manufacturer string | nil,
---@field model string | nil,
---This value is in millihertz (mHz) not hertz (Hz)
---@field refresh_rate number,
---@field is_primary boolean,
---@field device string,
---@field device_id string,
---@field id number,
---@field name string,
---@field scale_factor MonitorScaleFactor,
local monitor_instance = {}

---@alias MonitorVec table<number, Monitor>

---Attaches a component on the monitor.
---
---!!! example
---
---    ```lua
---    local monitors = hitokage.monitor.get_all()
---    
---    for _, monitor in ipairs(monitors) do
---      monitor:attach({
---        children = {
---          { Box = {} },
---          { Workspace = { halign = "Center", item_height = 22, item_width = 22 } },
---          { Clock = { format = "%a %b %u %r", halign = 'End' } },
---        },
---      })
---    end
---    ```
---
---@param props BarProps
---@return Bar
function monitor_instance:attach(props) end

-------------------------------------------------------------------------------

return monitor
