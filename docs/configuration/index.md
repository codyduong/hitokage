# Configuration

*hitokage* is configured with lua and css[^1]. *hitokage* by default looks for your configuration in
`%USERPROFILE%/.config/hitokage` and it looks for `init.lua` and `styles.css`.

The minimal configuration might look something like this:

`init.lua`
```lua
local monitors = hitokage.monitor.get_all()

for _, monitor in ipairs(monitors) do
  monitor:attach({
    children = {
      { Workspace = { halign = "Start", item_height = 24, item_width = 24 } },
      { Box = {} },
      { Clock = { format = "%a %b %u %r", halign = "End" } },
    },
  })
end
```

!!! danger

    This represents a full unsandboxed lua runtime! It can perform any action any lua runtime can
    including arbitrary code execution. As such, you should audit any configuration files you
    retrieve online.

`styles.css`
```css
.bar {
  background-color: rgba(29, 32, 33, 0.0);
  color: #f2e5bc;
  
  font-family: 'MesloLGS NF', 'Courier New', 'Bars', 'Font Awesome 5 Free';
  font-size: 12px;
  line-height: 12px;
  min-height: 24px;
}

.workspace {
  padding: 0px 0px 0px 0px;
  /* border: 1px solid rgba(255, 0, 0, 0.4); */
}

.workspacechild {
  /* distance between workspace children */
  /* margin-left: 8px; */
  border: 1px solid rgba(168, 153, 132, 0.4);

  color: #f2e5bc;
  background-color: rgba(60, 56, 54, 0.4);

  font-size: 11px;
  /* border-radius: 11px; */

  transition: all 500ms;
  transition-property: min-width, background-color;
}

.workspacechild:first-child {
  margin-left: 0px;
}

.workspacechild:selected {
  /* min-width: 32px; */
  border: 1px solid #f2e5bc;
  background-color: #f2e5bc;
  color: #333333;
  font-weight: bold;
}
```

Once you can have done this, hitokage can be run from the command line or powershell

```powershell
hitokage
```

[^1]: More details can be found here: [about/features](../features.html)

## Setting up Lua Language Server

!!! warning "🚧In progress🚧"

    This feature is not matured. It will require a clone of the repository.

_hitokage_ comes with type definitions for lua using [_lua-language-server_ (LuaLS)](https://luals.github.io/). It is located at [`hitokage-lua-lib`](https://github.com/codyduong/hitokage/tree/master/hitokage-lua-lib).

Once installed, you can configure LuaLS to add the type annotations for _hitokage_. You can follow [LuaLS's Configuration Guide](https://luals.github.io/wiki/configuration/), or an abridged version for configuring the addition of _hitokage_'s type definitions is below:

You will have to clone the entire repo, then point your `workspace.library` at `.\hitokage-lua-lib`

!!! info "LuaLS addon system"

    LuaLS comes with support for [addons](https://luals.github.io/wiki/addons/). Once _hitokage_ reaches v0.1.0 an addon will be published
    that will be available through LuaLS's addon system for _hitokage_'s type definitions. This is
    best suited for VSCode users who have installed the [sumneko.lua](https://marketplace.visualstudio.com/items?itemName=sumneko.lua) addon.
