---@meta hitokage.components.weather

--------------------------------------------------------------------------------
---Links to WeatherProps in 'hitokage-core\src\components\weather.rs'

---@class WeatherProps : BaseProps
-- @codyduong TODO add some descriptions here
---
---@field format string | ReactiveString | fun(forecast: WeatherForecast): string
---
---@field latitude number?
---
---@field longitude number?
---
---@field icons table<string, string>?

--------------------------------------------------------------------------------
---Links to WeatherUserData in 'hitokage-lua\src\components\weather.rs'

---@class Weather : Base
---
---@field type 'Weather'
local weather_instance = {}

---Get the type of widget
---@return 'Weather'
function weather_instance:get_type() end

---Get the format string
---@return string
function weather_instance:get_format() end

---Get the reactive format string
---@return ReactiveString
---@nodiscard
function weather_instance:get_format_reactive() end

---Set the format string
---@param string string
---@return nil
function weather_instance:set_format(string) end

--------------------------------------------------------------------------------
---Links to WeatherForecast in 'hitokage-lua\src\components\weather.rs'

---@class WeatherForecast
---
---@field temperature number
---@field wind_speed number
---@field wind_direction number
---@field weather_code number
---@field is_day boolean
