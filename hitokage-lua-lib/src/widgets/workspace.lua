--- @meta hitokage.widgets.workspace

-------------------------------------------------------------------------------
--- Links to WorkspaceProps in 'hitokage-core\src\widgets\workspace.rs'
--- @class WorkspaceProps
--- @field item_width integer?
--- @field item_height integer?
--- @field halign Align?
--- @field class CssClass?

-------------------------------------------------------------------------------
--- Links to hitokage-lua\src\widgets\workspace.rs
--- @class Workspace
---
--- @field type 'Workspace'
---
local workspace_instance = {}

--- Get the type of widget
--- @return 'Workspace'
function workspace_instance:get_type() end

--- Get the css classes
--- @return table<number, string>
function workspace_instance:get_class() end

--- Set the css classes
--- @param class CssClass
--- @return nil
function workspace_instance:set_class(class) end

--- Get the halign
--- @return Align
function workspace_instance:get_halign() end

--- Set the halign
--- @param halign Align
--- @return nil
function workspace_instance:set_halign(halign) end

--- Get the item width
--- @return integer
function workspace_instance:get_item_height() end

--- Set the item width
--- @param height integer
--- @return nil
function workspace_instance:set_item_height(height) end

--- Get the item width
--- @return integer
function workspace_instance:get_item_width() end

--- Set the item width
--- @param width integer
--- @return nil
function workspace_instance:set_item_width(width) end

-------------------------------------------------------------------------------
--- Represents the workspace module
--- @class workspacelib
local workspace = {}

return workspace