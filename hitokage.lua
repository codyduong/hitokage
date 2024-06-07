hitokage.debug(hitokage);

local monitors = hitokage.monitor.all()
local primary = hitokage.monitor.primary()

hitokage.debug(monitors, primary)

for _, display in pairs(monitors) do
  hitokage.bar.create({
    geometry = display.geometry
  })
end

-- local s = hitokage.read_state()
-- hitokage.debug(s);

while true do
  -- read subscriptions we setup earlier
  -- run subscriptions by checking diff
  local new = hitokage.event.new();
  -- if there is a new state please read it
  if new then
    local unread_states = hitokage.event.read();
    for _, state in pairs(unread_states) do
      -- hitokage.debug("wo")
      hitokage.debug("checking " .. state.event.type);
      if state.event.type == "FocusWorkspaceNumber" then
        hitokage.debug("we changed workspaces to " .. state.event.content);
      end
    end
  end
  hitokage.sleep_ms(100);
end
