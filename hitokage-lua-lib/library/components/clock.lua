---@meta hitokage.components.clock

--------------------------------------------------------------------------------
---Links to ClockProps in 'hitokage-core\src\components\clock.rs'

---A native component within `hitokage` that displays date and time information.
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
---    ---@type ClockProps
---    clock_props = { format = "%a %b %u %r" }
--- 
---    monitor:attach({
---      children = {
---        Clock = clock_props, 
---      },
---    })
---    ```
---
---The mounted API is documented here: <!--@mkdocs-ignore-start-->[`Clock`](lua://Clock)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/Clock" title="Clock">`Clock`</a> -->
---
---@class ClockProps : BaseProps
---
---A [chrono::format::strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) template string.
---
---@field format string | ReactiveString

--------------------------------------------------------------------------------
---Links to ClockUserData hitokage-lua\src\components\clock.rs

---A userdata which corresponds to the mounted version of <!--@mkdocs-ignore-start-->[`ClockProps`](lua://ClockProps)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/WrapClockProps/ClockProps" title="ClockProps">`ClockProps`</a> -->
---
---> A native component within `hitokage` that displays date and time information.
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
---    ---@type ClockProps
---    clock_props = { id = "clock1", format = "%a %b %u %r" }
--- 
---    bar = monitor:attach({
---      children = {
---        Clock = clock_props, 
---      },
---    })
--- 
---    clock1 = bar:get_child_by_id("clock1")
---    ```
---
---@class Clock : Base
---
---@field type 'Clock'
local clock_instance = {}

---Get the type of widget
---@return 'Clock'
function clock_instance:get_type() end

---Get the format string
---@return string
function clock_instance:get_format() end

---Get the reactive format string
---@return ReactiveString
---@nodiscard
function clock_instance:get_format_reactive() end

---Set the format string
---@param string string
---@return nil
function clock_instance:set_format(string) end
