---@meta hitokage.components.box

---@alias ComponentProps table<number, WidgetBatteryProps | WidgetBoxProps | WidgetClockProps | WidgetCpuProps | WidgetIconProps | WidgetLabelProps | WidgetMemoryProps | WidgetWeatherProps | WidgetWorkspaceProps>?

-------------------------------------------------------------------------------
---Links to BoxProps in 'hitokage-core\src\components\box.rs'
---@class BoxProps : BaseProps
---
---An array of components.
---
---!!! example
---
---    ```lua
---    children = {
---      { Box = {} },
---      { Workspace = { halign = "Center", item_height = 22, item_width = 22 } },
---      { Clock = { format = "%a %b %u %r", halign = 'End' } },
---    },
---    ```
---
---@field children ComponentProps
---
---!!! danger
---
---    Use [`children`](lua://BoxProps.children) instead
---
---!!! example
---
---    ```lua
---    widgets = {
---      { Box = {} },
---      { Workspace = { halign = "Center", item_height = 22, item_width = 22 } },
---      { Clock = { format = "%a %b %u %r", halign = 'End' } },
---    },
---    ```
---
---@deprecated
---@field widgets ComponentProps

-------------------------------------------------------------------------------
---Links to BoxUserData hitokage-lua\src\components\box.rs
---@class Box : Base
---
---@field type 'Box'
local box_instance = {}

---Get the type of widget
---@return 'Box'
function box_instance:get_type() end

---Returns whether the box is homogeneous (all children are the same size).
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Box.get_homogeneous.html)
---
---@return boolean
function box_instance:get_homogeneous() end

---Sets whether or not all children of `box` are given equal space in the box.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Box.set_homogeneous.html)
---
---@param homogeneous boolean
---@return nil
function box_instance:set_homogeneous(homogeneous) end

---Get the children on the box.
---@return table<number, Battery | Box | Clock | Cpu | Icon | Label | Weather | Workspace>
function box_instance:get_children() end

---Gets the first item in the widget tree that has the identifier.
---
---When `recursive` is set to `true` the search is performed breadth-first,
---then in order of components on the tree.
---
---@param id string The identifier
---@param recursive boolean? Defaults to `false`
---@return nil | Battery | Box | Clock | Cpu | Icon | Label | Weather | Workspace>
function box_instance:get_child_by_id(id, recursive) end

---!!! danger "Deprecated"
---
---    Use [`get_children`](lua://Box.get_children) instead.
---
---Get the components on the box.
---@deprecated
---@return table<number, Battery | Box | Clock | Cpu | Icon | Label | Weather | Workspace>
function box_instance:get_widgets() end

---!!! danger "Deprecated"
---
---    Use [`get_child_by_id`](lua://Box.get_child_by_id) instead.
---
---Gets the first item in the widget tree that has the identifier.
---
---When `recursive` is set to `true` the search is performed breadth-first,
---then in order of components on the tree.
---
---@deprecated
---@param id string The identifier
---@param recursive boolean? Defaults to `false`
---@return nil | Battery | Box | Clock | Cpu | Icon | Label | Weather | Workspace>
function box_instance:get_widget_by_id(id, recursive) end
