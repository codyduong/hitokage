--- @meta hitokage.widgets.box

-------------------------------------------------------------------------------
--- Links to BoxProps in 'hitokage-core\src\widgets\box.rs'
--- @class BoxProps : BaseProps
---
--- Whether to expand horizontally. Defaults to `true`
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.hexpand.html)
---
--- @field hexpand boolean?
---
--- Whether the children should all be the same size. Defaults to `true`
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Box.homogeneous.html)
---
--- @field homogeneous boolean?
---
--- Whether to expand vertically. Defaults to `true`
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.vexpand.html)
---
--- @field vexpand boolean?
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
--- @field widgets table<number, WidgetBoxProps | WidgetClockProps | WidgetIconProps | WidgetLabelProps | WidgetWorkspaceProps>?

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
--- @return table<number, Box | Clock | Label | Workspace>
function box_instance:get_widgets() end
