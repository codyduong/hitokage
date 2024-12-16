---@meta hitokage.api.bar

-------------------------------------------------------------------------------
---
---Represents the bar module
---@class bar
local bar = {}

---<!--@mkdocs-ignore-next-line-->
---**Deprecated:** Use [`monitor:attach`](lua://Monitor.attach) instead.
---<!--@mkdocs-include
---    !!! danger "Deprecated"
---
---    Use <a href="/hitokage/api/Monitor#method-attach">`monitor:attach`</a> instead.
----->
---
---Creates a new bar on a specified monitor.
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example-->
---
---    ```lua
---    local monitors = hitokage.monitor.get_all()
---
---    for _, monitor in ipairs(monitors) do
---      hitokage.bar.create(monitor, {
---        children = {
---          { Box = {} },
---          { Workspace = { halign = "Center", item_height = 22, item_width = 22 } },
---          { Clock = { format = "%a %b %u %r", halign = 'End' } },
---        },
---      })
---    end
---    ```
---
---@deprecated
---@param monitor Monitor
---@param bar_props BarProps
---@return Bar
function bar.create(monitor, bar_props) end

return bar
