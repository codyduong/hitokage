# hitokage 日と影

> [!CAUTION]
> hitokage is a work-in-progress, mostly built for personal use until I can dedicate more time to it.
> 
> hitokage will have sparse support and documentation, good luck.
>
> hitokage is in a pre-release state, the APIs are subject to breaking changes as I am still experimenting (this will not be the case after 0.1.0).
>
> hitokage does not have any readily available releases for download, you must build it yourself.

*__hitokage__ is a configurable status bar for Windows implemented in Rust using the relm4/gtk4 framework.*

It is primarily built for usage with [ `komorebi` ](https://github.com/LGUG2Z/komorebi), but is also usable as a 
standalone drop-in replacement for the default Windows Taskbar.

Documentation and guides at: ~~nowhere LOL!~~ (🚧 TODO USER FACING DOCS 🚧)

## Demo

![Demonstration of default hitokage status bar](media/demo.png)

## Configuration

*hitokage* is configured with lua and css<sup>[1](#css-footnote)</sup>. *hitokage* by default looks for your configuration in
`%USERPROFILE%/.config/hitokage` and it looks for `init.lua` and `styles.css`. An example configuration is found at [`example`](example).

The minimal configuration might look something like this:

`init.lua`
```lua
local monitors = hitokage.monitor.get_all()

for _, monitor in ipairs(monitors) do
  hitokage.bar.create(monitor, {
    widgets = {
      { Box = {} },
      { Workspace = { halign = "Center", item_height = 22, item_width = 22 } },
      { Clock = { format = "%a %b %u %r", halign = 'End' } },
    },
  })
end
```

`styles.css`
```
.bar {
  background-color: rgba(29, 32, 33, 0.0);
  color: #f2e5bc;
  
  font-family: 'MesloLGS NF', 'Courier New', 'Bars', 'Font Awesome 5 Free';
  font-size: 14px;
}

.workspace {
  padding: 0px 24px 0px 24px;
  /* border: 1px solid rgba(255, 0, 0, 0.4); */
}

.workspacechild {
  /* distance between workspace children */
  margin-left: 8px;
  border: 1px solid rgba(168, 153, 132, 0.4);

  color: #f2e5bc;
  background-color: rgba(60, 56, 54, 0.4);

  font-size: 11px;
  border-radius: 11px;

  transition: all 500ms;
  transition-property: min-width, background-color;
}

.workspacechild:first-child {
  margin-left: 0px;
}

.workspacechild:selected {
  min-width: 32px;
  border: 1px solid #f2e5bc;
  background-color: #f2e5bc;
  color: #333333;
  font-weight: bold;
}
```

*hitokage* also comes with *hitokage-lua-lib* for EmmyLua typings, and can be used with your LSP for helping you write your configuration.
<!-- The *hitokage-lua-lib* rockspec ~~is available at: ~~ (🚧 TODO ROCKSPEC 🚧) -->

__<a name="css-footnote">1</a>__: gtk4 css supported properties can be found here: https://docs.gtk.org/gtk4/css-properties.html

## Installation

<!--
* Install from nightly or stable from [releases]()
* Winget `winget install hitokage`
* Powershell Gallery `Install-Module hitokage`
-->

### Developing/Building From Source

Requires
* https://github.com/Relm4/Relm4/tree/main
  + https://gtk-rs.org/gtk4-rs/git/book/installation_windows.html
  + https://github.com/wingtk/gvsbuild#development-environment

<!--
Build notes:
* msys2 pkg-config sucks -> https://github.com/rust-lang/pkg-config-rs/issues/51#issuecomment-346300858
-->

## Acknowledgements
- [`yasb`](https://github.com/da-rth/yasb) - The original inspiration for this status bar
- [`komorebi`](https://github.com/LGUG2Z/komorebi) - The tiling manager used in conjunction with this status bar
- [`ButteryTaskbar2`](https://github.com/LuisThiamNye/ButteryTaskbar2) - Hiding the default windows taskbar
- [`wezterm`](https://github.com/wez/wezterm) - Code for various WinAPI and mlua utilities

## License

MIT
