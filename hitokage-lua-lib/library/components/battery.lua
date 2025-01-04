---@meta hitokage.components.battery

--------------------------------------------------------------------------------
---Links to BatteryProps in 'hitokage-core\src\components\battery.rs'

---A native component within `hitokage` that displays battery information.
---
---See <!--@mkdocs-ignore-start-->[`ComponentProps`](lua://ComponentProps)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/ComponentProps" title="ComponentProps">`ComponentProps`</a> -->
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
---
---    ```lua
---    monitor = hitokage.monitor.get_primary()
--- 
---    ---@type BatteryProps
---    battery_props = { format = "{{icon}}" }
--- 
---    monitor:attach({
---      children = {
---        Battery = battery_props, 
---      },
---    })
---    ```
---
---The mounted API is documented here: <!--@mkdocs-ignore-start-->[`Battery`](lua://Battery)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/Battery" title="Battery">`Battery`</a> -->
---
---@class BatteryProps : BaseProps
---
---A handlebars template string or function accepts BatteryInfo and returns a handlebars template string
---
---@codyduong TODO explain this
---
---@field format string | ReactiveString | fun(battery_info: BatteryInfo): string
---
---@field icons table<string, string>?

--------------------------------------------------------------------------------
---Links to BatteryUserData hitokage-lua\src\components\battery.rs

---A userdata which corresponds to the mounted version of <!--@mkdocs-ignore-start-->[`BatteryProps`](lua://BatteryProps)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/WrapBatteryProps/BatteryProps" title="BatteryProps">`BatteryProps`</a> -->
---
---> A native component within `hitokage` that displays battery information.
---
---This userdata can be retrieved using:
---<!--@mkdocs-ignore-start-->
---* [`Box:get_child_by_id`](lua://Box.get_child_by_id)
---* [`Box:get_children`](lua://Box.get_children)
---* [`Bar:get_child_by_id`](lua://Bar.get_child_by_id)
---* [`Bar:get_children`](lua://Bar.get_children)
---<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include * <a href="/hitokage/api/Box#method-get_child_by_id" title="Box#method-get_child_by_id">`Box:get_child_by_id`</a>
---* <a href="/hitokage/api/Box#method-get_children" title="Box#method-get_children">`Box:get_children`</a>
---* <a href="/hitokage/api/Bar#method-get_child_by_id" title="Bar#method-get_child_by_id">`Bar:get_child_by_id`</a>
---* <a href="/hitokage/api/Bar#method-get_children" title="Bar#method-get_children">`Bar:get_children`</a>
----->
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
---
---    ```lua
---    monitor = hitokage.monitor.get_primary()
--- 
---    ---@type BatteryProps
---    battery_props = { id = "battery1", format = "{{icon}}" }
--- 
---    bar = monitor:attach({
---      children = {
---        Battery = battery_props, 
---      },
---    })
--- 
---    battery1 = bar:get_child_by_id("battery1")
---    ```
---
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

--------------------------------------------------------------------------------
---Links to BatteryInfo hitokage-core\src\structs\system.rs

---@class BatteryInfo
---
---A value from 0.0 to 1.0 as a measure of battery capacity
---@field capacity number
---
---The estimated remaining time left in seconds
---@field seconds_left number
