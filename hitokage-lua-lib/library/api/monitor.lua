---@meta hitokage.api.monitor

-------------------------------------------------------------------------------
---Represents the monitor module
---@class monitor
local monitor = {}

---Retrieve all monitors on the device
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
---
---    ```lua
---    local monitors = hitokage.monitor.get_all()
---
---    hitokage.debug(monitors)
---    ```
---
---@return MonitorVec #An array of Monitor userdatas
function monitor.get_all() end

---@return Monitor #A Monitor userdata
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

---
---@class MonitorScaleFactor
---@field x number,
---@field y number,

-------------------------------------------------------------------------------
---Links to 'pub struct Monitor' in 'hitokage-core\src\lua\monitor.rs'

---The return result of
---<!--@mkdocs-ignore-next-line-->
---[`monitor`](lua://monitor)
---<!--@mkdocs-include <a href="/hitokage/api/hitokage/monitor" title="hitokage.monitor">`monitor`</a> -->
---functions
---
---<!--@mkdocs-ignore-start-->
---* [`monitor.get_all`](lua://monitor.get_all)
---* [`monitor.get_primary`](lua://monitor.get_primary)
---<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include * <a href="/hitokage/api/hitokage/monitor#function-get_all" title="hitokage.monitor#function-get_all">`monitor.get_all`</a>
---* <a href="/hitokage/api/hitokage/monitor#function-get_primary" title="hitokage.monitor#function-get_primary">`monitor.get_primary`</a>
----->
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
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
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
---@param self Monitor A monitor userdata
---@param props BarProps A table specifying the shape of the bar
---@return Bar #A Bar userdata
function monitor_instance.attach(self, props) end

-------------------------------------------------------------------------------

return monitor
