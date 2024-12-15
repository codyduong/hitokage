---@meta hitokage.components.icon

--------------------------------------------------------------------------------
---Links to IconProps in 'hitokage-core\src\components\icon.rs'

---@class IconProps : BaseProps
---
---A path to the file to display.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/property.Image.file.html)
---
---@field file string | ReactiveString

--------------------------------------------------------------------------------
---Links to IconUserData hitokage-lua\src\components\icon.rs

---@class Icon : Base
---
---@field type 'Icon'
local icon_instance = {}

---Get the type of widget
---@return 'Icon'
function icon_instance:get_type() end

---Gets the path to the file the icon is displaying.
---
---@return nil
function icon_instance:get_file() end

---Fetches the reactive file for the icon.
---
---@return string
---@nodiscard
function icon_instance:get_file_reactive() end

---Sets the icon from from the a path for displaying.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Image.set_from_file.html)
---
---@param string string
---@return string
function icon_instance:set_file(string) end
