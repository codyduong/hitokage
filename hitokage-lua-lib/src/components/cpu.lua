--- @meta hitokage.components.cpu

-------------------------------------------------------------------------------
--- Links to CpuProps in 'hitokage-core\src\components\cpu.rs'
--- @class CpuProps : BaseProps
-- @codyduong TODO add some descriptions here
---
--- @field format string | ReactiveString

-------------------------------------------------------------------------------
--- Links to CpuUserData hitokage-lua\src\components\cpu.rs
--- @class Cpu : Base
---
--- @field type 'Cpu'
local cpu_instance = {}

--- Get the type of widget
--- @return 'Cpu'
function cpu_instance:get_type() end

--- Get the format string
--- @return string
function cpu_instance:get_format() end

--- Get the reactive format string
--- @return ReactiveString
--- @nodiscard
function cpu_instance:get_format_reactive() end

--- Set the format string
--- @param string string
--- @return nil
function cpu_instance:set_format(string) end
