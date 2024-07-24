hitokage.debug("5");

-- hitokage.debug(hitokage);

local monitors = hitokage.monitor.get_all()
local primary = hitokage.monitor.get_primary()

-- hitokage.debug(monitors)

--- @type BarArray
local bars = {}

for _, monitor in ipairs(monitors) do
  if (monitor.model == "LG SDQHD") then
    goto continue
  end

  -- TODO better idiomatic syntax
  -- monitor.create_bar({
  --   widgets = {
  --     {Workspace = {}},
  --     {Clock = {format = "%Y-%m-%d %H:%M:%S"}},
  --     {Box = {}},
  --   }
  -- })

  table.insert(bars, hitokage.bar.create(monitor, {
    widgets = {
      { Box = {} },
      { Workspace = { halign = "Start" } },
      { Clock = { format = "%Y-%m-%d %I:%M:%S %p" } },
    },
    width = monitor.geometry.width - 16,
    offset = {
      x = 8,
      y = 8,
    }
  }))
  ::continue::
end


--- @alias WorkspaceTable table<number, Workspace>
--- @type WorkspaceTable
local workspaces = {}

--- @alias ClockTable table<number, Clock>
--- @type ClockTable
local clocks = {}

for i, bar in ipairs(bars) do
  while not bar:is_ready() do
    hitokage.debug("waiting for bar to instantiate", i)
    coroutine.yield() -- yield to other processes to occur
  end
  for _, widget in ipairs(bar:get_widgets()) do
    hitokage.debug(widget)
    if widget.type == "Clock" then
      table.insert(clocks, widget)
    end
    if widget.type == "Workspace" then
      table.insert(workspaces, widget)
    end 
  end
end

local timeout = function (timeout, action)
  return coroutine.create(function()
    local start_time = os.clock()
  
    while true do

      local elapsed_time = (os.clock() - start_time) * 1000
      local remaining_time = timeout - elapsed_time
      if (remaining_time <= 0) then
        start_time = os.clock()
        action()
      end

      coroutine.yield()
    end
  end)
end

-- local halign_test = timeout(1000, function()
--   for _, workspace in ipairs(workspaces) do
--     local halign = workspace:get_halign()

--     if halign == 'Start' then
--       workspace:set_halign('Center')
--     else
--       workspace:set_halign('Start')
--     end
--   end
-- end)

-- local clock_swapper = timeout(1000, function()
--   for _, clock in ipairs(clocks) do
--     local format = clock:get_format()
--     local current_hour = tonumber(os.date("%H"))
--     local sleep = false
--     if current_hour >= 0 and current_hour < 8 then
--       sleep = true
--     else
--       sleep = false
--     end
--     if sleep and format == "%Y-%m-%d %H:%M:%S" then
--       clock:set_format("🛌 SLEEP TIME 🛌")
--     else
--       clock:set_format("%Y-%m-%d %H:%M:%S")
--     end
--   end
-- end)

-- local s = hitokage.read_state()
-- hitokage.debug(s);

-- while true do
--   -- read subscriptions we setup earlier
--   -- run subscriptions by checking diff
--   local new = hitokage.event.has_unread();
--   -- if there is a new state please read it
--   if new then
--     local unread_states = hitokage.event.get_unread();
--     for _, state in pairs(unread_states) do
--       -- hitokage.debug("wo")
--       hitokage.debug("checking " .. state.event.type);
--       if state.event.type == "FocusWorkspaceNumber" then
--         hitokage.debug("we changed workspaces to " .. state.event.content);
--       end
--     end
--   end
--   -- preventer_fn();
--   hitokage.sleep_ms(100);
-- end

_subscribers = { "komorebi" }
_subscriptions = {
  komorebi = {}
}

local subscribe = function(name, callback)
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

-- subscribe("komorebi", function (unread_states)
--   for _, state in pairs(unread_states) do
--     hitokage.debug("checking " .. state.event.type);
--     if state.event.type == "FocusWorkspaceNumber" then
--       hitokage.debug("we changed workspaces to " .. state.event.content);
--     end
--   end
-- end)

-- subscribe("komorebi", function (unread_states)
--   -- for _, state in pairs(unread_states) do
--   --   hitokage.debug("checking " .. state.event.type);
--   --   if state.event.type == "TitleUpdate" then
--   --     hitokage.debug("we updated title to", state.event.content);
--   --   end
--   -- end
--   hitokage.debug(unread_states[#unread_states])
-- end)

local is_connection = function(obj)
  return type(obj) == "table" and obj.send ~= nil and obj.receive ~= nil
end

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
        _not_deadlocked();
        coroutine.yield(res_array);
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
    _not_deadlocked();
    coroutine.yield();

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

local komorebic_coroutine = coroutine.create(
  function()
    local subscriptions = rawget(rawget(_G, "_subscriptions"), "komorebi")
    if #subscriptions == 0 then
      return
    end

    while true do
      local new = hitokage.event.has_unread();
      if new then
        local unread_states = hitokage.event.get_unread();
        -- for _, state in pairs(unread_states) do
        --   hitokage.debug("checking " .. state.event.type);
        --   if state.event.type == "FocusWorkspaceNumber" then
        --     hitokage.debug("we changed workspaces to " .. state.event.content);
        --   end
        -- end
        for id, callback in pairs(subscriptions) do
          local status, res = pcall(callback, unread_states)
          if status == false then
            hitokage.error("Error running subscription callback {" .. id .. "}:", res)
          end
        end
      end
      coroutine.yield()
    end
  end
)

local file_watcher = coroutine.create(
  function()
    while true do
      local new = hitokage.event.configuration.changed()
      if new then
        coroutine.yield("Reload")
        -- stop running lua, then it will be passed to rust to reload this lua
      end
      coroutine.yield()
    end
  end
)

dispatcher({
  -- hitokage.loop.coroutine(),
  -- clock_swapper,
  -- halign_test,
  file_watcher,
  komorebic_coroutine,
})
