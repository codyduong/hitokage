--- @meta hitokage.widgets.weather

-------------------------------------------------------------------------------
--- Links to WeatherProps in 'hitokage-core\src\widgets\weather.rs'
--- @class WeatherProps : BaseProps
--- @field format string | ReactiveString?

-------------------------------------------------------------------------------
--- Links to WeatherUserData hitokage-lua\src\widgets\weather.rs
--- @class Weather : Base
---
--- @field type 'Weather'
local weather_instance = {}

--- Get the type of widget
--- @return 'Weather'
function weather_instance:get_type() end

--- Get the format string
--- @return string
function weather_instance:get_format() end

--- Get the reactive format string
--- @return ReactiveString
--- @nodiscard
function weather_instance:get_format_reactive() end

--- Set the format string
--- @param string string
--- @return nil
function weather_instance:set_format(string) end
