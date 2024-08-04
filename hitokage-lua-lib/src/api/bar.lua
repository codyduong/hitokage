--- @meta hitokage.api.bar

-------------------------------------------------------------------------------
--- Represents the bar module
--- @class barlib
local bar = {}

--- Creates a new bar on a specified monitor.
--- 
--- **Example**
--- ```lua
--- local monitors = hitokage.monitor.get_all()
--- 
--- for _, monitor in ipairs(monitors) do
---   hitokage.bar.create(monitor, {
---     widgets = {
---       { Box = {} },
---       { Workspace = { halign = "Center", item_height = 22, item_width = 22 } },
---       { Clock = { format = "%a %b %u %r", halign = 'End' } },
---     },
---   })
--- end
--- ```
--- 
--- @param monitor Monitor
--- @param bar_props BarProps
--- @return Bar
function bar.create(monitor, bar_props) end

return bar
