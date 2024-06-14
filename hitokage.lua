hitokage.debug(hitokage);

local monitors = hitokage.monitor.get_all()
local primary = hitokage.monitor.get_primary()

-- hitokage.debug(monitors, primary)

local bars = {}
for _, monitor in pairs(monitors) do
  table.insert(bars, hitokage.bar.create({
    geometry = monitor.geometry,
    widgets = {
      {Workspace = {}},
      {Clock = {format = "your_format_string"}},
      {Box = {}},
    },
  }))
end
for i, bar in ipairs(bars) do
  while not bar:is_ready() do
    hitokage.debug("waiting", i)
    coroutine.yield() -- yield ensures minimum of 100ms
  end
  hitokage.debug("ready", bar)
end

-- local s = hitokage.read_state()
-- hitokage.debug(s);

-- while true do
--   -- read subscriptions we setup earlier
--   -- run subscriptions by checking diff
--   local new = hitokage.event.has_unread();
--   -- if there is a new state please read it
--   if new then
--     local unread_states = hitokage.event.unread();
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

_subscribers = {"komorebi"}

function subscribe(name, callback)
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

subscribe("komorebi", function (unread_states)
  for _, state in pairs(unread_states) do
    hitokage.debug("checking " .. state.event.type);
    if state.event.type == "FocusWorkspaceNumber" then
      hitokage.debug("we changed workspaces to " .. state.event.content);
    end
  end
end)

subscribe("komorebi", function (unread_states)
  for _, state in pairs(unread_states) do
    hitokage.debug("checking " .. state.event.type);
    if state.event.type == "TitleUpdate" then
      hitokage.debug("we updated title to", state.event.content);
    end
  end
end)

function is_connection(obj)
  return type(obj) == "table" and obj.send ~= nil and obj.receive ~= nil
end

function dispatcher(threads)
  local min_time_ms = 100

  while true do
    local n = #threads
    
    if n == 0 then
      break
    end

    -- local start_time = os.clock()

    local connections = {}
    for i=1, n do
      local status, res = coroutine.resume(threads[i])
      if not status then
        hitokage.debug("Thread " .. i .. " exited:", res)
        table.remove(threads, i)
        break
      elseif is_connection(res) then
        table.insert(connections, res)
      end
      if #connections == n then
        socket.select(connections)
      end
    end

    -- local elapsed_time = (os.clock() - start_time) * 1000
    -- local remaining_time = min_time_ms - elapsed_time

    -- if remaining_time > 0 then
    --   -- hitokage.debug("Yielding back " .. remaining_time)
    --   hitokage.sleep_ms(remaining_time)
    -- end

    _not_deadlocked();
  end
end

komorebic_coroutine = coroutine.create(
  function ()
    local subscriptions = rawget(rawget(_G, "_subscriptions"), "komorebi")
    if #subscriptions == 0 then
      return
    end

    while true do
      local new = hitokage.event.has_unread();
      if new then
        local unread_states = hitokage.event.unread();
        -- for _, state in pairs(unread_states) do
        --   hitokage.debug("checking " .. state.event.type);
        --   if state.event.type == "FocusWorkspaceNumber" then
        --     hitokage.debug("we changed workspaces to " .. state.event.content);
        --   end
        -- end
        for id, callback in pairs(subscriptions) do
          local status, res = pcall(callback, unread_states)
          if status == false then
            hitokage.error("Error running callback {" .. id .. "}:", res)
          end
        end
      end
      coroutine.yield()
    end
  end
)

dispatcher({
  -- hitokage.loop.coroutine(),
  komorebic_coroutine,
})