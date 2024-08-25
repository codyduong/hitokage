--- @meta hitokage.widgets.workspace

-------------------------------------------------------------------------------
--- Links to WorkspaceProps in 'hitokage-core\src\widgets\workspace.rs'
--- @class WorkspaceProps : BaseProps
-- @codyduong TODO add some descriptions here
---
--- @field item_width integer?
--- @field item_height integer?
--- @field format string?

-------------------------------------------------------------------------------
--- Links to hitokage-lua\src\widgets\workspace.rs
--- @class Workspace
---
--- @field type 'Workspace'
local workspace_instance = {}

--- Get the type of widget
--- @return 'Workspace'
function workspace_instance:get_type() end

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
