---@meta hitokage.components.base

--------------------------------------------------------------------------------
---Links to BaseProps in 'hitokage-core\src\components\base.rs'

---@class BaseProps
---
---Optional css class names. Either space-delimited classes, or an array of
---class names. Built-in hitokage components will always contain their type as
---a class name.
---
---<!--@mkdocs-include
---!!! example
---
---    ```lua
---    class = "red blue green"
---    ```
---    or
---    ```lua
---    class = {"red", "blue", "green"}
---    ```
----->
---[View gtk4 documentation](https://docs.gtk.org/gtk4/css-properties.html)
---
---@field class string | table<number, string>?
---
---Sets the horizontal alignment of `widget`. Defaults to `'Fill'`
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.halign.html)
---
---@field halign Align?
---
---Override for height request of the widget.
---If this is `-1`, the natural request will be used.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.height-request.html)
---
---@field height_request integer?
---
---A unique identifier.
---
---This is not enforced or checked by hitokage, it is simply used in utility methods such
---as <!--@mkdocs-ignore-start-->[`get_child_by_id`](lua://Bar.get_child_by_id).<!--@mkdocs-ignore-end-->
---<!--@mkdocs-include <a href="/hitokage/api/Bar#method-get_child_by_id">`Bar:get_child_by_id`</a>
--- or <a href="/hitokage/api/Box#method-get_child_by_id">`Box:get_child_by_id`.</a>-->
---
---@field id string?
---
---Whether to expand horizontally. Defaults to `false`
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.hexpand.html)
---
---@field hexpand boolean?
---
---Sets the vertical alignment of `widget`. Defaults to `'Fill'`.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.valign.html)
---
---@field valign Align?
---
---Whether to expand vertically. Defaults to `false`
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.vexpand.html)
---
---@field vexpand boolean?
---
---Override for width request of the widget.
---If this is `-1`, the natural request will be used.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/property.Widget.width-request.html)
---
---@field width_request integer?

--------------------------------------------------------------------------------
---Links to BoxUserData hitokage-lua\src\components\box.rs

---Abstract base userdata every other userdata extends from.
---@class Base
local base_instance = {}

---Get the css classes as an array of strings.
---
---@return table<number, string>
function base_instance:get_class() end

---Set the css classes either as a space delimited string or array of strings.
---
---@param ... string
---@return nil
---@overload fun(class: table<number, string>): nil
function base_instance:set_class(...) end

---Gets the horizontal alignment of `widget`.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_halign.html)
---
---@return Align
function base_instance:get_halign() end

---Sets the horizontal alignment of `widget`.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_halign.html)
---
---@param halign Align
---@return nil
function base_instance:set_halign(halign) end

---Returns the content height of the widget.
---
---To learn more about widget sizes, see the coordinate system [overview](https://docs.gtk.org/gtk4/coordinates.html).
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_height.html)
---
---@return integer
function base_instance:get_height() end

---Gets the height request that was explicitly set for the widget using <!--@mkdocs-ignore-next-line-->
---[`set_height_request`](lua://Base.set_height_request) or [`set_size_request`](lua://Base.set_size_request).
---<!--@mkdocs-include <a href="#method-set_height_request">`set_height_request`</a> or <a href="#method-set_size_request">`set_size_request`</a>.-->
---
---A value of `-1` stored in height indicates that it has not been set explicitly and the natural requisition of the widget will be used instead.
---To get the height a widget will actually request, call [`measure_height`](lua://Base.measure_height) instead of this function.
---
---See also: <!--@mkdocs-ignore-next-line-->
---[`get_width_request`](lua://Base.get_width_request), [`get_size_request`](lua://Base.get_size_request).
---<!--@mkdocs-include <a href="#method-get_width_request">`get_width_request`</a>, <a href="#method-get_size_request">`get_size_request`</a>.-->
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_size_request.html)
---
---@return integer
function base_instance:get_height_request() end

---Sets the minimum height of a widget.
---
---That is, the widget’s size request will be at least `height`. You can use this function to force a widget to be taller than it normally would be.
---
---If the height request in a given direction is `-1` (unset), then the “natural” height request of the widget will be used instead.
---
---See also: <!--@mkdocs-ignore-next-line-->
---[`set_width_request`](lua://Base.set_width_request), [`set_size_request`](lua://Base.set_size_request).
---<!--@mkdocs-include <a href="#method-set_width_request">`set_width_request`</a>, <a href="#method-set_size_request">`set_size_request`</a>.-->
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_size_request.html)
---
---@param height integer?
---@return nil
function base_instance:set_height_request(height) end

---Gets whether the widget would like any available extra horizontal space.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_hexpand.html)
---
---@return boolean
function base_instance:get_hexpand() end

---Sets whether the widget would like any available extra horizontal space.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_hexpand.html)
---
---@param hexpand boolean
---@return nil
function base_instance:set_hexpand(hexpand) end

---Gets the size request that was explicitly set for the widget using <!--@mkdocs-ignore-next-line-->
---[`set_size_request`](lua://Base.set_size_request).
---<!--@mkdocs-include <a href="#method-set_size_request">`set_size_request`</a>.-->
---
---A value of `-1` stored in width or height indicates that that dimension has not been set explicitly and the natural requisition of the widget will be used instead.
---To get the size a widget will actually request, call [`measure`](lua://Base.measure) instead of this function.
---
---See also: <!--@mkdocs-ignore-next-line-->
---[`get_width_request`](lua://Base.get_width_request), [`get_height_request`](lua://Base.get_height_request).
---<!--@mkdocs-include <a href="#method-get_width_request">`get_width_request`</a>, <a href="#method-get_height_request">`get_height_request`</a>.-->
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_size_request.html)
---
---@return [integer, integer]
function base_instance:get_size_request() end

---Sets the minimum size of a widget.
---
---That is, the widget’s size request will be at least `width` by `height`. You can use this function to force a widget to be larger than it normally would be.
---
---If the size request in a given direction is `-1` (unset), then the “natural” size request of the widget will be used instead.
---
---See also: <!--@mkdocs-ignore-next-line-->
---[`set_width_request`](lua://Base.set_width_request), [`set_height_request`](lua://Base.set_height_request).
---<!--@mkdocs-include <a href="#method-set_width_request">`set_width_request`</a>, <a href="#method-set_height_request">`set_height_request`</a>.-->
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_size_request.html)
---
---@param size [integer?, integer?]
---@return nil
function base_instance:set_size_request(size) end

---Gets the vertical alignment of `widget`.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_valign.html)
---
---@return Align
function base_instance:get_valign() end

---Sets the vertical alignment of `widget`.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_valign.html)
---
---@param valign Align
---@return nil
function base_instance:set_valign(valign) end

---Gets whether the widget would like any available extra vertical space.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_vexpand.html)
---
---@return boolean
function base_instance:get_vexpand() end

---Sets whether the widget would like any available extra vertical space.
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_vexpand.html)
---
---@param vexpand boolean
---@return nil
function base_instance:set_vexpand(vexpand) end

---Returns the content width of the widget.
---
---To learn more about widget sizes, see the coordinate system [overview](https://docs.gtk.org/gtk4/coordinates.html).
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_width.html)
---
---@return integer
function base_instance:get_width() end

---Gets the width request that was explicitly set for the widget using <!--@mkdocs-ignore-next-line-->
---[`set_width_request`](lua://Base.set_width_request) or [`set_size_request`](lua://Base.set_size_request).
---<!--@mkdocs-include <a href="#method-set_width_request">`set_width_request`</a> or <a href="#method-set_size_request">`set_size_request`</a>.-->
---
---A value of `-1` stored in width indicates that it has not been set explicitly and the natural requisition of the widget will be used instead.
---To get the width a widget will actually request, call [`measure_width`](lua://Base.measure_width) instead of this function.
---
---See also: <!--@mkdocs-ignore-next-line-->
---[`get_height_request`](lua://Base.get_height_request), [`get_size_request`](lua://Base.get_size_request).
---<!--@mkdocs-include <a href="#method-get_height_request">`get_height_request`</a>, <a href="#method-get_size_request">`get_size_request`</a>.-->
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.get_size_request.html)
---
---@return integer
function base_instance:get_width_request() end

---Sets the minimum width of a widget.
---
---That is, the widget’s size request will be at least `width`. You can use this function to force a widget to be wider than it normally would be.
---
---If the width request in a given direction is `-1` (unset), then the “natural” width request of the widget will be used instead.
---
---See also: <!--@mkdocs-ignore-next-line-->
---[`set_height_request`](lua://Base.set_height_request), [`set_size_request`](lua://Base.set_size_request).
---<!--@mkdocs-include <a href="#method-set_height_request">`set_height_request`</a>, <a href="#method-set_size_request">`set_size_request`</a>.-->
---
---[View gtk4 documentation](https://docs.gtk.org/gtk4/method.Widget.set_size_request.html)
---
---@param width integer?
---@return nil
function base_instance:set_width_request(width) end
