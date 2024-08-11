--- @meta hitokage.widgets.icon

-------------------------------------------------------------------------------
--- Links to IconProps in 'hitokage-core\src\widgets\icon.rs'
--- @class IconProps : BaseProps
---
--- The contents of the icon.
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Icon.icon.html)
---
--- @field icon string

-------------------------------------------------------------------------------
--- Links to IconUserData hitokage-lua\src\widgets\icon.rs
--- @class Icon : Base
---
--- @field type 'Icon'
local icon_instance = {}

--- Get the type of widget
--- @return 'Icon'
function icon_instance:get_type() end

--- Gets the file for the icon.
---
--- @return nil
function icon_instance:get_file() end

--- Fetches the reactive file for the icon.
---
--- @return string
--- @nodiscard
function icon_instance:get_file_reactive() end

--- Sets the icon from file.
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Image.set_from_file.html)
--- 
--- @param string string
--- @return string
function icon_instance:set_file(string) end
