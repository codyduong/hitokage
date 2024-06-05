local displays = hitokage.display.all()

hitokage.debug(displays)

hitokage.bar.create()

for _, display in pairs(displays) do
  -- hitokage.debug(display)
  -- hitokage.create_widget();
  -- hitokage.bar.create() --// @TODO @codyduong
end

local s = hitokage.read_state()
hitokage.debug(s);

while true do
  -- read subscriptions we setup earlier
  -- run subscriptions by checking diff
  local new = hitokage.new_state();
  -- if there is a new state please read it
  if new then
    local unread_states = hitokage.read_state();
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
