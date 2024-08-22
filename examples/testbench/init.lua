local monitors = hitokage.monitor.get_all()

--- @type BarArray
local bars = {}

local reactive_labels = {}
local reactive_imgs = {}

local clock_icons = {
	"\u{F144A}",
	"\u{F143F}",
	"\u{F1440}",
	"\u{F1441}",
	"\u{F1442}",
	"\u{F1443}",
	"\u{F1444}",
	"\u{F1445}",
	"\u{F1446}",
	"\u{F1447}",
	"\u{F1448}",
	"\u{F1449}",
}

--- @type table<number, ReactiveString>
local reactive_clock_icons = {}

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
	local reactive_label = hitokage.unstable.reactive.create("foo \u{EECB}")
	local reactive_img = hitokage.unstable.reactive.create("./smiley.png")
	local reactive_clock_icon = hitokage.unstable.reactive.create(clock_icons[tonumber(os.date("%H")) % 12 + 1])

	table.insert(reactive_labels, reactive_label)
	table.insert(reactive_imgs, reactive_img)
	table.insert(reactive_clock_icons, reactive_clock_icon)

	local mem_str =
	'{{pad "left" (round (div used 1024) 1) 4}} ({{ pad "left" (concat (round (mult (div used total) 100) 1) "%") 4 }})'
	local cpu_str = '{{pad "left" (concat (round (mult usage 100) 1) "%") 6}}'

	-- .. 'C1: {{pad "right" (concat (round (mult core1_usage 100) 1) "%") 6}}'
	-- .. 'C1: {{pad "right" (concat (round (mult core1_usage 100) 1) "%") 6}}'
	-- .. 'C2: {{pad "right" (concat (round (mult core2_usage 100) 1) "%") 6}}'
	-- .. 'C3: {{pad "right" (concat (round (mult core3_usage 100) 1) "%") 6}}'
	-- .. 'C4: {{pad "right" (concat (round (mult core4_usage 100) 1) "%") 6}}'
	-- .. 'C5: {{pad "right" (concat (round (mult core5_usage 100) 1) "%") 6}}'
	-- .. 'C6: {{pad "right" (concat (round (mult core6_usage 100) 1) "%") 6}}'
	-- .. 'C7: {{pad "right" (concat (round (mult core7_usage 100) 1) "%") 6}}'

	table.insert(
		bars,
		hitokage.bar.create(monitor, {
			widgets = {
				{
					Box = {
						hexpand = false,
						homogeneous = true,
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
				{ Workspace = { halign = "Center", item_height = 22, item_width = 22, format = "{{add index 1}}" } },
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
				{
					Box = {
						homogeneous = false,
						halign = "End",
						class = "right_bar",
						widgets = {
							{ Label = { label = "\u{E0B2}", class = "data_start" } },
							{
								Box = {
									homogeneous = false,
									halign = "Fill",
									class = "data_wrapper",
									widgets = {
										{ Weather = { latitude = 38.95773795883854, longitude = -95.25382422045898, hexpand = true } },
										{ Label = { label = "\u{EFC5}", class = "icon memory" } },
										{ Memory = { format = mem_str, halign = "End" } },
										{ Label = { label = "\u{F4BC}", class = "icon cpu", id = "test1" } },
										{ Cpu = { format = cpu_str, halign = "End" } },
										{ Label = { label = "\u{E0B2}", class = "clock_start", halign = "End" } },
										
									},
								},
							},
							{
								Box = {
									hexpand = false,
									homogeneous = false,
									class = "clock_wrapper",
									widgets = {
										{ Label = { label = "\u{F00ED}", class = "icon clock" } },
										{ Clock = { format = "%a %b %e", halign = "End" } },
										{ Label = { label = reactive_clock_icon, class = "icon clock" } },
										{ Clock = { format = "%r", halign = "End" } },
									},
								},
							},
							{ Label = { label = "\u{E0B4}", class = "bar_end" } },
						},
					},
				},
			},
			homogeneous = true,
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

	local label = bar:get_widget_by_id("test1", true)
	local old_label = label:get_label()
	-- hitokage.debug("fooface", label)
	local routine = hitokage.timeout(1000, function()
		local current_label = label:get_label()
		-- hitokage.debug(current_label)
		if current_label == "\u{F4BC}" then
			label:set_label("a")
		else
			label:set_label("\u{F4BC}")
		end
	end)
	hitokage.dispatch(routine)
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

local label_reactor = hitokage.timeout(1000, function()
	local current_format = reactive_labels[1]:get()
	if current_format == "foo \u{EECB}  \u{F0E0}" then
		reactive_labels[1]:set("demo a b")
	else
		reactive_labels[1]:set("foo \u{EECB} \u{F0E0}")
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

local update_clock_icon = hitokage.timeout(1000, function()
	for _, clock_icon in ipairs(reactive_clock_icons) do
		-- local current_icon = clock_icon:get();
		-- for i, icon in ipairs(clock_icons) do
		-- 	if current_icon == icon then
		-- 		local next_icon = clock_icons[(i % #clock_icons) + 1]
		-- 		clock_icon:set(next_icon)
		-- 		break
		-- 	end
		-- end
		local hour_24 = tonumber(os.date("%H"))
		local hour_12 = hour_24 % 12
		clock_icon:set(clock_icons[hour_12 + 1])
	end
end)

-- hitokage.dispatch(format_reactor)
hitokage.dispatch(label_reactor)
hitokage.dispatch(img_reactor)
hitokage.dispatch(css_boxes_test)
hitokage.dispatch(update_clock_icon)
