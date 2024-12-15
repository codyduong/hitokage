---@meta hitokage.api.reactive

-------------------------------------------------------------------------------

---Represents the reactive userdata for String type
---@class ReactiveString
---@field value string
local reactive_string_userdata = {}

---Get the value of the reactive variable
---@return string
function reactive_string_userdata:get() end

---Set the value of the reactive variable
---@param value string
---@return nil
function reactive_string_userdata:set(value) end

-------------------------------------------------------------------------------
-- ---Represents the monitor module for Bool type
-- ---@class ReactiveBool
-- local reactive_bool_userdata = {}

-- ---Get the value of the reactive variable
-- ---@return boolean
-- function reactive_bool_userdata:get() end

-- ---Set the value of the reactive variable
-- ---@param value boolean
-- ---@return nil
-- function reactive_bool_userdata:set(value) end

-------------------------------------------------------------------------------
---Represents the reactive module
---@class reactive
local reactive = {}

---Creates a pointer to an rust `Arc<Mutex<T>>`. Currently only supports strings.
---
---@param value string
---@return ReactiveString
function reactive.create(value) end

-- ---@param value boolean
-- ---@return ReactiveBool
-- function reactive.create(value) end

return reactive
