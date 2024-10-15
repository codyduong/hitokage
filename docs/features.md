# Features

## Programmatic Configuration

_hitokage_'s most notable feature is its usage of a lua runtime to configure and customize your taskbar.
This allows for the programmatic logic and adjustability of your config on any amount of things, moreso
than most status bars out there.

More specifically _hitokage_ uses [Lua5.4](https://www.lua.org/versions.html), allowing all modern lua constructs.

## Styling and Appearance

_hitokage_ supports css styling. 

More specifically _hitokage_ uses [Gtk 4.14.4](https://gitlab.gnome.org/GNOME/gtk) under the hood, which
uses a subset of css. View [Gtk – 4.0: GTK CSS Properties](https://docs.gtk.org/gtk4/css-properties.html) for more information.

## Components

_hitokage_ ships with a number of built-in components that make setting up your status bar easier.
Custom components can also be created.

* [Workspace](./lua/workspace.html)
    - Used with [_komorebi_](https://github.com/LGUG2Z/komorebi) to indicate current workspace the user is in
* [Clock](./lua/clock.html)
* [Battery](./lua/battery.html)
* [CPU](./lua/cpu.html)
* [Memory](./lua/memory.html)
* [Weather](./lua/weather.html)

Primitive components

* [Box](./lua/box.html)
* [Label](./lua/label.html)
* [Icon](./lua/icon.html)
