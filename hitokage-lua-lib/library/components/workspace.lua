---@meta hitokage.components.workspace

--------------------------------------------------------------------------------
---Links to WorkspaceProps in 'hitokage-core\src\components\workspace.rs'

---A native component within `hitokage` that displays workspace information.
---
---This is in the particular context of running `hitokage` with a window/tiling manager:
---
---* [`komorebi`](https://github.com/LGUG2Z/komorebi)
---* [TODO (not supported) `glazewm`](https://github.com/glzr-io/glazewm)
---
---See <!--@mkdocs-ignore-start-->[`ComponentProps`](lua://ComponentProps)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/ComponentProps" title="ComponentProps">`ComponentProps`</a> -->
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
---
---    ```lua
---    monitor = hitokage.monitor.get_primary()
---
---    ---@type WorkspaceProps
---    workspace_props = { Workspace = { halign = "Start", item_height = 24, item_width = 24 } }
---
---    monitor:attach({
---      children = {
---        Workspace = workspace_props,
---      },
---    })
---    ```
---
---The mounted API is documented here: <!--@mkdocs-ignore-start-->[`Workspace`](lua://Workspace)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/Workspace" title="Workspace">`Workspace`</a> -->
---
---@class WorkspaceProps : BaseProps
-- @codyduong TODO add some descriptions here
---
---@field item_width integer?
---@field item_height integer?
---@field format string?

--------------------------------------------------------------------------------
---Links to hitokage-lua\src\components\workspace.rs

---A userdata which corresponds to the mounted version of <!--@mkdocs-ignore-start-->[`WorkspaceProps`](lua://WorkspaceProps)<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/WrapWorkspaceProps/WorkspaceProps" title="WorkspaceProps">`WorkspaceProps`</a> -->
---
---> A native component within `hitokage` that displays workspace information.
--->
---> This is in the particular context of running `hitokage` with a window/tiling manager:
--->
---> * [`komorebi`](https://github.com/LGUG2Z/komorebi)
---> * [TODO (not supported) `glazewm`](https://github.com/glzr-io/glazewm)
---
---This userdata can be retrieved using:
---<!--@mkdocs-ignore-start-->
---* [`Box:get_child_by_id`](lua://Box.get_child_by_id)
---* [`Box:get_children`](lua://Box.get_children)
---* [`Bar:get_child_by_id`](lua://Bar.get_child_by_id)
---* [`Bar:get_children`](lua://Bar.get_children)
---<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include * <a href="/hitokage/api/Box#method-get_child_by_id" title="Box#method-get_child_by_id">`Box:get_child_by_id`</a>
---* <a href="/hitokage/api/Box#method-get_children" title="Box#method-get_children">`Box:get_children`</a>
---* <a href="/hitokage/api/Bar#method-get_child_by_id" title="Bar#method-get_child_by_id">`Bar:get_child_by_id`</a>
---* <a href="/hitokage/api/Bar#method-get_children" title="Bar#method-get_children">`Bar:get_children`</a>
----->
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
---
---    ```lua
---    monitor = hitokage.monitor.get_primary()
---
---    ---@type WorkspaceProps
---    workspace_props = { id = "workspace1", Workspace = { halign = "Start", item_height = 24, item_width = 24 } }
---
---    bar = monitor:attach({
---      children = {
---        Workspace = workspace_props,
---      },
---    })
---
---    workspace1 = bar:get_child_by_id("workspace1")
---    ```
---
---@class Workspace
---
---@field type 'Workspace'
local workspace_instance = {}

---Get the type of widget
---@return 'Workspace'
function workspace_instance:get_type() end

---Get the item width
---@return integer
function workspace_instance:get_item_height() end

---Set the item width
---@param height integer
---@return nil
function workspace_instance:set_item_height(height) end

---Get the item width
---@return integer
function workspace_instance:get_item_width() end

---Set the item width
---@param width integer
---@return nil
function workspace_instance:set_item_width(width) end
