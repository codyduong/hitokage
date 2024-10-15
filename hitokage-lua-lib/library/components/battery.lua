---@meta hitokage.components.battery

-------------------------------------------------------------------------------
---Links to BatteryProps in 'hitokage-core\src\components\battery.rs'
---@class BatteryProps : BaseProps
-- @codyduong TODO add some descriptions here
---
---@field format string | ReactiveString | fun(battery_info: BatteryInfo): string
---
---@field icons table<string, string>?

-------------------------------------------------------------------------------
---Links to BatteryUserData hitokage-lua\src\components\battery.rs
---@class Battery : Base
---
---@field type 'Battery'
local battery_instance = {}

---Get the type of widget
---@return 'Battery'
function battery_instance:get_type() end

---Get the format string
---@return string
function battery_instance:get_format() end

---Get the reactive format string
---@return ReactiveString
---@nodiscard
function battery_instance:get_format_reactive() end

---Set the format string
---@param string string
---@return nil
function battery_instance:set_format(string) end

-------------------------------------------------------------------------------
---Links to BatteryInfo hitokage-core\src\structs\system.rs
---@class BatteryInfo
---
---A value from 0.0 to 1.0 as a measure of battery capacity
---@field capacity number
---
---The estimated remaining time left in seconds
---@field seconds_left number
