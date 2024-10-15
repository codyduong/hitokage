# hitokage 日と影

*__hitokage__ is a configurable status bar for Windows implemented in Rust using the [relm4](https://github.com/Relm4/Relm4) GUI library.*

It is primarily built for usage with [ `komorebi` ](https://github.com/LGUG2Z/komorebi), but is also usable as a 
standalone drop-in replacement for the default Windows Taskbar.

Documentation and guides at: [codyduong.github.io/hitokage](https://codyduong.github.io/hitokage/)

## Demos

[`examples/minimal`](/examples/minimal/):
![Demonstration of a minimal hitokage status bar](/docs/media/minimal.png)
<br/>

[`examples/testbench`](/examples/testbench/):
![Demonstration of the testbench hitokage status bar](/docs/media/testbench.gif)

[`codyduong`](https://github.com/codyduong/dotfiles/tree/ba4eb2b9044646ab1b33797dd9b11f2bc1a6ea4d/windows/.files/%25USERPROFILE%25/.config/hitokage):
![Demonstration of codyduong's hitokage status bar](/docs/media/codyduong.png)

## Configuration

*hitokage* is configured with lua and css<sup>[1](#css-footnote)</sup>. *hitokage* by default looks for your configuration in
`%USERPROFILE%/.config/hitokage` and it looks for `init.lua` and `styles.css`. Example configurations are found at [`examples`](/examples/).

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

*hitokage* also comes with [*hitokage-lua-lib*](/hitokage-lua-lib/) to provide [LuaLS](https://github.com/luals/lua-language-server) type annotations, and can be used with your preferred editor for helping you write your configuration.
<!-- The *hitokage-lua-lib* rockspec ~~is available at: ~~ (🚧 TODO ROCKSPEC 🚧) -->

---

__<a name="css-footnote">1</a>__: [gtk4 css supported properties](https://docs.gtk.org/gtk4/css-properties.html)

## Installation

Install from [nightly](https://github.com/codyduong/hitokage/releases/nightly) or [latest](https://github.com/codyduong/hitokage/releases/latest) from the [releases page](https://github.com/codyduong/hitokage/releases).

> [!CAUTION]
> hitokage is in a pre-release state, the APIs are subject to breaking changes (this will not be the case after 0.1.0).

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
- [`wezterm`](https://github.com/wez/wezterm) - Code for various WinAPI and mlua utilities
<!-- - [`ButteryTaskbar2`](https://github.com/LuisThiamNye/ButteryTaskbar2) - Hiding the default windows taskbar -->

## License

MIT
