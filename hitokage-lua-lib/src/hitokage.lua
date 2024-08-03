--- @meta hitokage

--- This is the global module for [hitokage](https://github.com/codyduong/hitokage)
---
--- [View documentation](https://codyduong.dev/hitokage/lua/hitokage)
---
--- @class hitokagelib
_G.hitokage = {}
_G._not_deadlocked = function() end
_G._subscribers = {}
_G._subscriptions = {}

local _common = require("hitokage.widgets.common")
local bar = require("hitokage.widgets.bar")
local _box = require("hitokage.widgets.box")
local _clock = require("hitokage.widgets.clock")
local monitor = require("hitokage.api.monitor")
local _workspace = require("hitokage.widgets.workspace")

-------------------------------------------------------------------------------
--- Utility functions

--- Output debug message to rust runtime
--- @vararg any
function hitokage.debug(...) end
--- Output error message to rust runtime
--- @vararg any
function hitokage.error(...) end
--- Output info message to rust runtime
--- @vararg any
function hitokage.info(...) end

--- Sleep function in milliseconds
--- @param ms number Amount of time to sleep.
function hitokage.sleep_ms(ms) end

-------------------------------------------------------------------------------
--- Compose hitokage
hitokage.bar = bar
hitokage.monitor = monitor
