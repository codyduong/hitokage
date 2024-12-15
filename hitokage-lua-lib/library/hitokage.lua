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

---This is the primary module for [hitokage](https://github.com/codyduong/hitokage)
---
---It is globally available as part of the lua environment variable: (`_G`)[lua://G]
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
---@param ms number Amount of time to sleep
function hitokage.sleep_ms(ms) end

-------------------------------------------------------------------------------
---Functions written in lua

---Add a coroutine to the hitokage event loop. A coroutine can run at most once
---per 100ms. This is an internal throttling mechanism that cannot be bypassed.
---
---**Example**:
---```lua
----- This function will log something every ~1000ms
---local interval = hitokage.timeout(1000, function()
---		hitokage.debug("About 1000ms has passed")
---end)
---hitokage.dispatch(interval)
---```
---@param thread_or_threads thread | table<number, thread>
---@return nil
function hitokage.dispatch(thread_or_threads) end

---Subscribe to an event loop dispatched by hitokage
---<!--@mkdocs-ignore-start-->
---<!--LuaLS doc generator creates an oprhaned code block that we don't want in mkdocs-->
---@param name 'komorebi'
---@param callback fun(notification: KomorebiNotification)
---@return nil
function hitokage.subscribe(name, callback) end

---Utility function to help create a coroutine that runs at regular intervals
---
---<!--@mkdocs-ignore-next-line-->
---**Example:**
---<!--@mkdocs-include
---    !!! example -->
---
---    ```lua
---    -- This function will log something every ~1000ms
---    local interval = hitokage.timeout(1000, function()
---    	hitokage.debug("About 1000ms has passed")
---    end)
---    hitokage.dispatch(interval)
---    ```
---
---<!--@mkdocs-include
---!!! warning
---
---    The return result must be dispatched with [`hitokage.dispatch`](#function-dispatch) in order to run
----->
---@param timeout number Number in milliseconds that must pass before the coroutine can be run again
---@param action function A callback function that is run once the `timeout` has passed
---@return thread
---@nodiscard
function hitokage.timeout(timeout, action) end

-------------------------------------------------------------------------------
---Compose hitokage

---Represents the unstable module. Contains experimental/unsafe modules.
---
---This code is exposed for testing purposes for user feedback. Or contains
---unsafe rust code which can cause crashses or memory leaks.
---
---@class unstable
local unstable = {}
unstable.reactive = reactive

hitokage.bar = bar
hitokage.monitor = monitor
hitokage.unstable = unstable
