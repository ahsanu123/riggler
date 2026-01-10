# Developing In Linux but Use Windows Library only

to use cross library and able to get help from it in neovim lsp, you need to install 
it target with rustup, for example to develop using `windows-rs` in linux, 
you need to install `x86_64-pc-windows-gnu` with command `rustup install x86_64-pc-windows-gnu`
then inside your `init.lua` (it use `rustaceanvim`)  you need specify which target you want use.  

so if you want hint from windows library you need enable `x86_64-pc-windows-gnu`, 

and then if you need hint for linux lib, you must enable `x86_64-unknown-linux-gnu`


its bit weird but at least i can work with windows library in linux

update this note if i found something better

```lua

require("config.lazy")

-- https://github.com/mrcjkb/rustaceanvim?tab=readme-ov-file#gear-advanced-configuration
vim.g.rustaceanvim = {
  server = {
    settings = {
      ["rust-analyzer"] = {
        cargo = {
          allTargets = true,
          target = "x86_64-pc-windows-gnu",
          -- target = "x86_64-unknown-linux-gnu",
        },
      },
    },
  },
}
```
