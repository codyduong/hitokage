--- @meta hitokage.api.bar

-------------------------------------------------------------------------------
--- Represents the bar module
--- @class barlib
local bar = {}

--- **Deprecated**. Use [`monitor:attach`](lua://monitorlib.attach) instead.
---
--- Creates a new bar on a specified monitor.
---
--- **Example**
--- ```lua
--- local monitors = hitokage.monitor.get_all()
---
--- for _, monitor in ipairs(monitors) do
---   hitokage.bar.create(monitor, {
---     children = {
---       { Box = {} },
---       { Workspace = { halign = "Center", item_height = 22, item_width = 22 } },
---       { Clock = { format = "%a %b %u %r", halign = 'End' } },
---     },
---   })
--- end
--- ```
---
--- @deprecated
--- @param monitor Monitor
--- @param bar_props BarProps
--- @return Bar
function bar.create(monitor, bar_props) end

return bar
