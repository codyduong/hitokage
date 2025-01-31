---@diagnostic disable: undefined-field

---@meta hitokage.append

-- ---@module 'hitokage'

_subscribers = { "komorebi" }
_subscriptions = {
	komorebi = {},
}

---@overload fun(name: 'komorebi', callback: fun(notification: KomorebiNotification)): nil
---@param name 'komorebi'
---@param callback fun(notification: KomorebiNotification)
---@return nil
_G.hitokage.subscribe = function(name, callback)
	local is_subscriber = false
	for _, approvedName in ipairs(_subscribers) do
		if name == approvedName then
			is_subscriber = true
			break
		end
	end

	-- Panic if the name is not approved
	if not is_subscriber then
		error("Name not approved")
	end

	local subscriptions = rawget(_G, "_subscriptions") or {}
	local subscriptions_sub = rawget(subscriptions, name) or {}
	table.insert(subscriptions_sub, callback)
	rawset(subscriptions, name, subscriptions_sub)
	rawset(_G, "_subscriptions", subscriptions)
end

local komorebic_coroutine = coroutine.create(function()
	local subscriptions = rawget(rawget(_G, "_subscriptions"), "komorebi")
	if #subscriptions == 0 then
		return
	end

	while true do
		local new = hitokage._internals.event.has_unread()
		if new then
			local unread_states = hitokage._internals.event.get_unread()
			for id, callback in pairs(subscriptions) do
				for _, state in pairs(unread_states) do
					local status, res = pcall(callback, state)
					if status == false then
						hitokage.error("Error running subscription callback {" .. id .. "}:", res)
					end
				end
			end
		end
		coroutine.yield()
	end
end)

local file_watcher = coroutine.create(function()
	while true do
		local new = hitokage._internals.event.configuration.changed()
		if new then
			coroutine.yield("Reload")
			-- stop running lua, then it will be passed to rust to reload this lua
			_G["threads"] = {}
		end
		coroutine.yield()
	end
end)

local callback_watcher = coroutine.create(function()
	while true do
		local actions = hitokage._internals.actions.get_unread()
		if actions ~= nil then
			for _, action in pairs(actions) do
				hitokage.error("my fridge ran")
				action:call()
			end
		end
		coroutine.yield()
	end
end)

_G["_threads"] = {
	komorebic_coroutine,
	file_watcher,
	callback_watcher,
}

---@param timeout number
---@param action function
---@return thread
_G.hitokage.timeout = function(timeout, action)
	return coroutine.create(function()
		local start_time = os.clock()

		while true do
			local elapsed_time = (os.clock() - start_time) * 1000
			local remaining_time = timeout - elapsed_time
			if remaining_time <= 0 then
				start_time = os.clock()
				action()
			end

			coroutine.yield()
		end
	end)
end

---@param thread_or_threads thread | table<number, thread>
---@return nil
_G.hitokage.dispatch = function(thread_or_threads)
	local function isCoroutine(thread)
		return type(thread) == "thread" and coroutine.status(thread) ~= nil
	end

	if isCoroutine(thread_or_threads) then
		table.insert(_G["_threads"], thread_or_threads)
	elseif type(thread_or_threads) == "table" then
		for _, thread in ipairs(thread_or_threads) do
			if not isCoroutine(thread) then
				error("One of the arguments in the array is not a coroutine", 2)
			end
			table.insert(_G["_threads"], thread)
		end
	else
		error("Argument is not a coroutine or array of coroutines", 2)
	end
end
