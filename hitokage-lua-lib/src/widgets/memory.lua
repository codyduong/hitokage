--- @meta hitokage.widgets.memory

-------------------------------------------------------------------------------
--- Links to MemoryProps in 'hitokage-core\src\widgets\memory.rs'
--- @class MemoryProps : BaseProps
-- @codyduong TODO add some descriptions here
---
--- @field format string | ReactiveString?

-------------------------------------------------------------------------------
--- Links to MemoryUserData hitokage-lua\src\widgets\memory.rs
--- @class Memory : Base
---
--- @field type 'Memory'
local memory_instance = {}

--- Get the type of widget
--- @return 'Memory'
function memory_instance:get_type() end

--- Get the format string
--- @return string
function memory_instance:get_format() end

--- Get the reactive format string
--- @return ReactiveString
--- @nodiscard
function memory_instance:get_format_reactive() end

--- Set the format string
--- @param string string
--- @return nil
function memory_instance:set_format(string) end
