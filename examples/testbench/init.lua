local monitors = hitokage.monitor.get_all()

--- @type BarArray
local bars = {}

local reactive_formats = {}
local reactive_labels = {}
local reactive_imgs = {}

for _, monitor in ipairs(monitors) do
	if monitor.model == "LG SDQHD" then
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

	-- the unsafe operation occurs in creating reactives in lua. this has to do with how we serialize data...
	local reactive_format = hitokage.unstable.reactive.create("%a %b %e %r")
	local reactive_label = hitokage.unstable.reactive.create("foo \u{EECB}  \u{F0E0}")
	local reactive_img = hitokage.unstable.reactive.create("./smiley.png")

	table.insert(reactive_formats, reactive_format)
	table.insert(reactive_labels, reactive_label)
	table.insert(reactive_imgs, reactive_img)

	local cpu_str = 'C0: {{pad "right" (round (mult core0_usage 100) 1) 5}}'
		.. 'C1: {{pad "right" (round (mult core1_usage 100) 1) 5}}'
		.. 'C2: {{pad "right" (round (mult core2_usage 100) 1) 5}}'
		.. 'C3: {{pad "right" (round (mult core3_usage 100) 1) 5}}'
		.. 'C4: {{pad "right" (round (mult core4_usage 100) 1) 5}}'
		.. 'C5: {{pad "right" (round (mult core5_usage 100) 1) 5}}'
		.. 'C6: {{pad "right" (round (mult core6_usage 100) 1) 5}}'
		.. 'C7: {{pad "right" (round (mult core7_usage 100) 1) 5}}'
		.. ' A: {{pad "right" (round (mult usage 100) 1) 6}}'

	table.insert(
		bars,
		hitokage.bar.create(monitor, {
			widgets = {
				{
					Box = {
						widgets = {
							{
								Box = {
									class = "red",
								},
							},
							{
								Box = {
									class = "green",
								},
							},
							{
								Box = {
									widgets = {},
									class = "blue",
								},
							},
							{
								Box = {
									class = { "red", "green blue" },
									homogeneous = false,
									widgets = {
										{
											Label = {
												label = reactive_label,
												hexpand = true,
											},
										},
										{
											Icon = {
												file = reactive_img,
												hexpand = false,
											},
										},
									},
								},
							},
						},
					},
				},
				-- { Box = {} },
				{ Workspace = { halign = "Center", item_height = 22, item_width = 22, format = "{{add index 1}}" } },
				{ Cpu = { format = cpu_str, halign = "End" } },
				-- { Box = {
				-- 	widgets = {
				-- 		{ Cpu = { format = 'C0: {{pad "right" (round (mult core0_usage 100) 1) 5}}', halign = "End" } },
				-- 		{ Cpu = { format = 'C1: {{pad "right" (round (mult core1_usage 100) 1) 5}}', halign = "End" } },
				-- 		{ Cpu = { format = 'C2: {{pad "right" (round (mult core2_usage 100) 1) 5}}', halign = "End" } },
				-- 		{ Cpu = { format = 'C3: {{pad "right" (round (mult core3_usage 100) 1) 5}}', halign = "End" } },
				-- 		{ Cpu = { format = 'C4: {{pad "right" (round (mult core4_usage 100) 1) 5}}', halign = "End" } },
				-- 		{ Cpu = { format = 'C5: {{pad "right" (round (mult core5_usage 100) 1) 5}}', halign = "End" } },
				-- 		{ Cpu = { format = 'C6: {{pad "right" (round (mult core6_usage 100) 1) 5}}', halign = "End" } },
				-- 		{ Cpu = { format = 'C7: {{pad "right" (round (mult core7_usage 100) 1) 5}}', halign = "End" } },
				-- 		{ Cpu = { format = ' A: {{pad "right" (round (mult usage 100) 1) 5}}', halign = "End" } },
				-- 	},
				-- 	hexpand = false,
				-- }},
				{ Clock = { format = reactive_format, halign = "End" } },
			},
			homogeneous = false,
			width = monitor.geometry.width - 16,
			offset = {
				x = 8,
				y = 8,
			},
		})
	)
	::continue::
end

--- @alias WorkspaceTable table<number, Workspace>
--- @type WorkspaceTable
local workspaces = {}

--- @alias ClockTable table<number, Clock>
--- @type ClockTable
local clocks = {}

--- @alias BoxesTable table<number, Box>
--- @type BoxesTable
local boxes = {}

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
		if widget.type == "Box" then
			table.insert(boxes, widget)
		end
	end
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

local css_boxes_test = hitokage.timeout(1000, function()
	local widgets = boxes[1]:get_widgets()

	local first = widgets[1]:get_class()
	for index, bar in ipairs(widgets) do
		if next(widgets, index) == nil then
			bar:set_class(first)
		else
			bar:set_class(widgets[index + 1]:get_class())
		end
	end
end)

local format_reactor = hitokage.timeout(1000, function()
	local current_format = reactive_formats[1]:get()
	if current_format == "%a %b %u %r" then
		reactive_formats[1]:set("demo demo demo")
	else
		reactive_formats[1]:set("%a %b %u %r")
	end
end)

local label_reactor = hitokage.timeout(1000, function()
	local current_format = reactive_labels[1]:get()
	if current_format == "foo \u{EECB}  \u{F0E0}" then
		reactive_labels[1]:set("demo demo demo")
	else
		reactive_labels[1]:set("foo \u{EECB}  \u{F0E0}")
	end
end)

local img_reactor = hitokage.timeout(1000, function()
	local current_format = reactive_imgs[1]:get()
	if current_format == "./smiley.png" then
		reactive_imgs[1]:set("")
	else
		reactive_imgs[1]:set("./smiley.png")
	end
end)

-- hitokage.dispatch(format_reactor)
-- hitokage.dispatch(label_reactor)
-- hitokage.dispatch(img_reactor)
-- hitokage.dispatch(css_boxes_test)
