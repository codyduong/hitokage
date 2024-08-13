local monitors = hitokage.monitor.get_all()

for _, monitor in ipairs(monitors) do
  hitokage.bar.create(monitor, {
    widgets = {
      { Workspace = { halign = "Start", item_height = 22, item_width = 22 } },
      { Box = {} },
      { Clock = { format = "%a %b %u %r", halign = 'End' } },
    },
  })
end