local monitors = hitokage.monitor.get_all()

for _, monitor in ipairs(monitors) do
	monitor:attach({
		widgets = {
			{ Workspace = { halign = "Start", item_height = 24, item_width = 24 } },
			{ Box = {} },
			{ Clock = { format = "%a %b %u %r", halign = "End" } },
		},
		homogeneous = true,
	})
end
