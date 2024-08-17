--- @meta hitokage.widgets.box

-------------------------------------------------------------------------------
--- Links to BoxProps in 'hitokage-core\src\widgets\box.rs'
--- @class BoxProps : BaseProps
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
--- @field widgets table<number, WidgetBoxProps | WidgetClockProps | WidgetCpuProps | WidgetIconProps | WidgetLabelProps | WidgetMemoryProps | WidgetWorkspaceProps>?

-------------------------------------------------------------------------------
--- Links to BoxUserData hitokage-lua\src\widgets\box.rs
--- @class Box : Base
---
--- @field type 'Box'
local box_instance = {}

--- Get the type of widget
--- @return 'Box'
function box_instance:get_type() end

--- Returns whether the box is homogeneous (all children are the same size).
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Box.get_homogeneous.html)
---
--- @return boolean
function box_instance:get_homogeneous() end

--- Sets whether or not all children of `box` are given equal space in the box.
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Box.set_homogeneous.html)
---
--- @param homogeneous boolean
--- @return nil
function box_instance:set_homogeneous(homogeneous) end

--- Get the widgets on the box
--- @return table<number, Box | Clock | Cpu | Icon | Label | Workspace>
function box_instance:get_widgets() end
