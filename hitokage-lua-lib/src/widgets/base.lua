--- @meta hitokage.widgets.base

-------------------------------------------------------------------------------
--- Links to BaseProps in 'hitokage-core\src\widgets\base.rs'
--- @class BaseProps
--- 
--- Optional css class names. Built-in hitokage widgets will always contain 
--- their type as a class name.
--- 
--- **Example**
--- ```lua
--- class = "red blue green" 
--- class = {"red", "blue", "green"}
--- -- These two are equivalent
--- ```
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/css-properties.html)
--- 
--- @field class CssClass?
--- 
--- Sets the horizontal alignment of `widget`. Defaults to `'Start'`
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.halign.html)
--- 
--- @field halign Align?
--- 
--- Whether to expand horizontally. Defaults to `false`
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.hexpand.html)
--- 
--- @field hexpand boolean?
--- 
--- Whether the children should all be the same size. Defaults to `false`
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Box.homogeneous.html)
--- 
--- @field homogeneous boolean?
---
--- Sets the vertical alignment of `widget`. Defaults to `'Start'`
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.valign.html)
--- 
--- @field valign Align?
--- 
--- Whether to expand vertically. Defaults to `false`
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.vexpand.html)
--- 
--- @field vexpand boolean?

-------------------------------------------------------------------------------
--- Links to BoxUserData hitokage-lua\src\widgets\box.rs
--- @class Base
local base_instance = {}




--- Get the css classes as an array of strings.
--- 
--- @return table<number, string>
function base_instance:get_class() end

--- Set the css classes either as a space delimited string or array of strings.
--- 
--- @param class CssClass
--- @return nil
function base_instance:set_class(class) end




--- Gets the horizontal alignment of `widget`.
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_halign.html)
--- 
--- @return Align
function base_instance:get_halign() end

--- Sets the horizontal alignment of `widget`.
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_halign.html)
--- 
--- @param halign Align
--- @return nil
function base_instance:set_halign(halign) end




--- Gets whether the widget would like any available extra horizontal space.
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_hexpand.html)
--- 
--- @return boolean
function base_instance:get_hexpand() end

--- Sets whether the widget would like any available extra horizontal space.
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_hexpand.html)
--- 
--- @param hexpand boolean
--- @return nil
function base_instance:set_hexpand(hexpand) end



--- Returns whether the box is homogeneous (all children are the same size).
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Box.get_homogeneous.html)
--- 
--- @return boolean
function base_instance:get_homogeneous() end

--- Sets whether or not all children of `box` are given equal space in the box.
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Box.set_homogeneous.html)
--- 
--- @param homogeneous boolean
--- @return nil
function base_instance:set_homogeneous(homogeneous) end




--- Gets the vertical alignment of `widget`.
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_valign.html)
--- 
--- @return Align
function base_instance:get_valign() end

--- Sets the vertical alignment of `widget`.
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_valign.html)
--- 
--- @param valign Align
--- @return nil
function base_instance:set_valign(valign) end




--- Gets whether the widget would like any available extra vertical space.
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_vexpand.html)
--- 
--- @return boolean
function base_instance:get_vexpand() end

--- Sets whether the widget would like any available extra vertical space.
--- 
--- [View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_vexpand.html)
--- 
--- @param hexpand boolean
--- @return nil
function base_instance:set_vexpand(hexpand) end