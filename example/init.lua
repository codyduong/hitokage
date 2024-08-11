local monitors = hitokage.monitor.get_all()

--- @type BarArray
local bars = {}

local reactives = {}

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
	local reactive_format = hitokage.unstable.reactive.create("%a %b %u %r")

	table.insert(reactives, reactive_format)

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
									widgets = {
										{
											Label = {
												label = "foo",
											},
										},
									},
								},
							},
						},
					},
				},
				-- { Box = {} },
				{ Workspace = { halign = "Center", item_height = 22, item_width = 22 } },
				{ Clock = { format = reactive_format, halign = "End" } },
			},
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

local css_boxes_test = hitokage.timeout(0, function()
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

local reactives = hitokage.timeout(500, function()
	local current_format = reactives[1]:get()
	if current_format == "%a %b %u %r" then
		reactives[1]:set("reactive demo")
	else
		reactives[1]:set("%a %b %u %r")
	end
end)

hitokage.dispatch(reactives)
-- hitokage.dispatch(css_boxes_test)
