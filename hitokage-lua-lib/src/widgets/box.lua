--- @meta hitokage.widgets.box

-------------------------------------------------------------------------------
--- Links to BoxProps in 'hitokage-core\src\widgets\box.rs'
--- @class BoxProps : BaseProps, HasChildrenProps
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

-------------------------------------------------------------------------------
--- Links to BoxUserData hitokage-lua\src\widgets\box.rs
--- @class Box : Base, HasChildren
--- 
--- @field type 'Box'
local box_instance = {}

--- Get the type of widget
--- @return 'Box'
function box_instance:get_type() end

--- Get the widgets on the box
--- @return table<number, Box | Clock | Workspace>
function box_instance:get_widgets() end
