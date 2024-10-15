---@meta hitokage.components.clock

-------------------------------------------------------------------------------
---Links to ClockProps in 'hitokage-core\src\components\clock.rs'
---@class ClockProps : BaseProps
---
---A string matching a [chrono::format::strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) string.
---
---@field format string | ReactiveString?

-------------------------------------------------------------------------------
---Links to ClockUserData hitokage-lua\src\components\clock.rs
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
