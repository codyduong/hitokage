--- @meta hitokage.widgets.box

-------------------------------------------------------------------------------
--- Links to BoxProps in 'hitokage-core\src\widgets\box.rs'
--- @class BoxProps : BaseProps, HasChildrenProps

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
