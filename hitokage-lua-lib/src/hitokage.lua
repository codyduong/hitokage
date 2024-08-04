--- @meta hitokage
--- 
--- @module 'hitokage.common'
--- @module 'hitokage.widgets.base'
--- @module 'hitokage.widgets.box'
--- @module 'hitokage.widgets.clock'
--- @module 'hitokage.widgets.common'
--- @module 'hitokage.widgets.workspace'

--- This is the global module for [hitokage](https://github.com/codyduong/hitokage)
---
--- @class hitokagelib
_G.hitokage = {}
_G._not_deadlocked = function() end
_G._subscribers = {}
_G._subscriptions = {}

local bar = require("hitokage.api.bar")
local monitor = require("hitokage.api.monitor")

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
