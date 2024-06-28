-------------------------------------------------------------------------------
--- Links to ClockProps in 'hitokage-core\src\widgets\clock.rs'
--- @class ClockProps
--- @fields format string?

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
--- @return nil
function clock_instance.set_format() end

-------------------------------------------------------------------------------
--- Represents the clock module
--- @class clocklib
local clock = {}

return clock
