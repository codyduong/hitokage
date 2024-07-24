-------------------------------------------------------------------------------
--- Links to ClockProps in 'hitokage-core\src\widgets\clock.rs'
--- @class ClockProps
--- @field format string?
--- @field halign Align?

-------------------------------------------------------------------------------
--- Links to ClockUserData hitokage-lua\src\widgets\clock.rs
--- @class Clock
---
--- @field type 'Clock'
---
--- Wrapper around bar:get_format() and set_format()
--- @field format string
---
local clock_instance = {}

--- Get the type of widget
--- @return 'Clock'
function clock_instance:get_type() end

--- Get the format string
--- @return string
function clock_instance:get_format() end

--- Get the format string
--- @param string string
--- @return nil
function clock_instance:set_format(string) end

--- Get the halign
--- @return Align
function clock_instance:get_halign() end

--- Set the halign
--- @param halign Align
--- @return nil
function clock_instance:set_halign(halign) end

-------------------------------------------------------------------------------
--- Represents the clock module
--- @class clocklib
local clock = {}

return clock
