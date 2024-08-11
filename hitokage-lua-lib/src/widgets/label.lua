--- @meta hitokage.widgets.label

-------------------------------------------------------------------------------
--- Links to LabelProps in 'hitokage-core\src\widgets\label.rs'
--- @class LabelProps : BaseProps
---
--- The contents of the label.
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Label.label.html)
---
--- @field label string

-------------------------------------------------------------------------------
--- Links to LabelUserData hitokage-lua\src\widgets\label.rs
--- @class Label : Base
---
--- @field type 'Label'
local label_instance = {}

--- Get the type of widget
--- @return 'Label'
function label_instance:get_type() end

--- Fetches the text from a label.
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Label.get_label.html)
---
--- @return string
function label_instance:get_label() end

--- Fetches the reactive text from a label.
---
--- @return string
--- @nodiscard
function label_instance:get_label_reactive() end

--- Sets the text of the label.
---
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Label.set_label.html)
---
--- @param string string
--- @return nil
function label_instance:set_label(string) end
