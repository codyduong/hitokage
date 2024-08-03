-------------------------------------------------------------------------------
--- Links to BoxProps in 'hitokage-core\src\widgets\box.rs'
--- @class BoxProps
--- @field widgets WidgetPropsVec?
--- @field class CssClass?

-------------------------------------------------------------------------------
--- Links to BoxUserData hitokage-lua\src\widgets\box.rs
--- @class Box
---
--- @field type 'Box'
---
--- Wrapper around bar:get_widgets()
--- @field widgets table<number, Box | Clock | Workspace>
---
local box_instance = {}

--- Get the type of widget
--- @return 'Box'
function box_instance:get_type() end

--- Get the css classes
--- @return table<number, string>
function box_instance:get_class() end

--- Set the css classes
--- @param class CssClass
--- @return nil
function box_instance:set_class(class) end

--- Get the widgets on the box
--- @return table<number, Box | Clock | Workspace>
function box_instance:get_widgets() end

-------------------------------------------------------------------------------
--- Represents the box module
--- @class boxlib
local box = {}

return box