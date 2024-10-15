---@meta hitokage.append

-- ---@module 'hitokage.prepend'

local dispatcher = function(threads)
	local min_time_ms = 100

	while true do
		local n = #threads

		if n == 0 then
			break
		end

		-- local start_time = os.clock()

		local connections = {}
		for i = 1, n do
			local status, res_array = coroutine.resume(threads[i])

			if status and res_array then
				-- immediately sending any message means we reset the cooldown timer
				_not_deadlocked()
				coroutine.yield(res_array)
			elseif not status then
				if res_array then
					hitokage.debug("Thread " .. i .. " exited:", res_array)
					table.remove(threads, i)
					break
				end
			end
			-- elseif is_connection(res) then
			--   table.insert(connections, res)
			-- end
			-- if #connections == n then
			--   socket.select(connections)
			-- end
		end

		-- ensure a minimum wait time of 100ms
		_not_deadlocked()
		coroutine.yield()

		-- local elapsed_time = (os.clock() - start_time) * 1000
		-- local remaining_time = min_time_ms - elapsed_time

		-- if remaining_time > 0 then
		--   -- hitokage.debug("Yielding back " .. remaining_time)
		--   hitokage.sleep_ms(remaining_time)
		-- end

		-- _not_deadlocked();
		-- coroutine.yield()
	end
end

dispatcher(rawget(_G, "_threads"))
