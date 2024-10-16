---@meta hitokage
---
---@module 'hitokage.common'
---@module 'hitokage.types.komorebi'
---@module 'hitokage.components.base'
---@module 'hitokage.components.box'
---@module 'hitokage.components.clock'
---@module 'hitokage.components.common'
---@module 'hitokage.components.weather'
---@module 'hitokage.components.workspace'

---This is the global module for [hitokage](https://github.com/codyduong/hitokage)
---
---@class hitokage
_G.hitokage = {}
_G._not_deadlocked = function() end
_G._subscribers = {}
_G._subscriptions = {}

local bar = require("hitokage.api.bar")
local monitor = require("hitokage.api.monitor")
local reactive = require("hitokage.api.reactive")

-------------------------------------------------------------------------------
---Utility functions

---Output debug message to rust runtime
---@vararg any
function hitokage.debug(...) end
---Output error message to rust runtime
---@vararg any
function hitokage.error(...) end
---Output info message to rust runtime
---@vararg any
function hitokage.info(...) end

---Sleep function in milliseconds
---@param ms number Amount of time to sleep.
function hitokage.sleep_ms(ms) end

-------------------------------------------------------------------------------
---Functions written in lua

---Add a coroutine to the hitokage event loop.
---
---All coroutines are run, then we buffer until 100ms has passed since the
---start of the first coroutine.
---
---@param thread_or_threads thread | table<number, thread>
---@return nil
function hitokage.dispatch(thread_or_threads) end

---@overload fun(name: 'komorebi', callback: fun(notification: KomorebiNotification)): nil
---@param name 'komorebi'
---@param callback fun(notification: KomorebiNotification)
---@return nil
function hitokage.subscribe(name, callback) end

---@param timeout number
---@param action function
---@return thread
function hitokage.timeout(timeout, action) end

-------------------------------------------------------------------------------
---Compose hitokage

---Represents the unstable module. Either it is experimental code that is
---exposed for testing purposes for user feedback, or contains code that uses
---unsafe rust, which can cause crashes.
---
---@class Unsafe
local unstable = {
	reactive = reactive,
}

hitokage.bar = bar
hitokage.monitor = monitor
hitokage.unstable = unstable
