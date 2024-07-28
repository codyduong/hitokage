package = "hitokage"
version = "0.1.0"
source = {
   url = "https://github.com/codyduong/hitokage",
   -- tag = "v0.1.0",
   branch = "dev",
   -- commit = "",
   dir = "hitokage-lua-lib",
}
description = {
   summary = "EmmyLua type annotations for hitokage",
   detailed = [[
      EmmyLua type annotations for hitokage. This luarocks package exports no working code.
   ]],
   homepage = "https://github.com/codyduong/hitokage",
   license = "MIT"
}
dependencies = {
   "lua >= 5.4"
}
build = {
   type = "builtin",
   modules = {
      ["hitokage"] = "src/hitokage.lua",
   }
}