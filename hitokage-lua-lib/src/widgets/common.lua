--- @meta hitokage.widgets.common

-------------------------------------------------------------------------------
--- Links to WidgetProps in 'hitokage-core\src\widgets\mod.rs'
--- 
--- @source hitokage-core\src\widgets\mod.rs
--- 
--- @class WidgetClockProps
--- @field Clock ClockProps
--- @class WidgetBoxProps
--- @field Box BoxProps
--- @class WidgetWorkspaceProps
--- @field Workspace WorkspaceProps

-------------------------------------------------------------------------------
--- @class HasChildrenProps
--- 
--- An array of widgets.
--- 
--- **Example**
--- ```lua
--- widgets = {
---   { Box = {} },
---   { Workspace = { halign = "Center", item_height = 22, item_width = 22 } },
---   { Clock = { format = "%a %b %u %r", halign = 'End' } },
--- },
--- ```
--- 
--- @field widgets table<number, WidgetBoxProps | WidgetClockProps | WidgetWorkspaceProps>

-------------------------------------------------------------------------------
--- Links to BoxUserData hitokage-lua\src\widgets\box.rs
--- @class HasChildren
--- 
--- Gets the attached widgets. 
--- @field widgets table<number, Box | Clock | Workspace>
local has_children_instance = {}

--- Gets the attached widgets.
--- @return table<number, Box | Clock | Workspace>
function has_children_instance:get_widgets() end
