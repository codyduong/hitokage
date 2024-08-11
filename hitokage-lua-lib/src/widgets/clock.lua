--- @meta hitokage.widgets.clock

-------------------------------------------------------------------------------
--- Links to ClockProps in 'hitokage-core\src\widgets\clock.rs'
--- @class ClockProps : BaseProps
--- @field format string | ReactiveString?
--- @field halign Align?

-------------------------------------------------------------------------------
--- Links to ClockUserData hitokage-lua\src\widgets\clock.rs
--- @class Clock : Base
---
--- @field type 'Clock'
local clock_instance = {}

--- Get the type of widget
--- @return 'Clock'
function clock_instance:get_type() end

--- Get the format string
--- @return ReactiveString
function clock_instance:get_format() end

--- Set the format string
--- @param string string | ReactiveString
--- @return nil
function clock_instance:set_format(string) end
