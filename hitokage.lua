local displays = hitokage.display.all()

hitokage.debug(displays)

for _, display in pairs(displays) do
  -- hitokage.debug(display)
  -- hitokage.create_widget();
  -- hitokage.bar.create() --// @TODO @codyduong
end

read_state(function (state)
  hitokage.debug(state)
end)
-- print(hitokage);
-- foo();
-- foo();
-- foo();
-- hitokage.print("foobarbaz");
-- print("womp");