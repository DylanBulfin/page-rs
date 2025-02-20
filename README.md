# page-rs
I've never managed to get `bat` or `less` to feel good, so this is my attempt at a simple pager in rust.

## Features
- Fully configurable keybindings (you probably want to change defaults)
- Basic left/down/up/right navigation
- Vim-style searching

## Installation
### Non-NixOS 
`cargo install --path .`

### NixOS
- On NixOS, with flakes enabled and home-manager as a NixOS module, make the following changes to your `flake.nix`;
```
# Add page-rs as flake input
inputs.page-rs.url = "gitub:DylanBulfin/page-rs";

# Apply the below overlay (more info: https://nixos.wiki/wiki/Overlays)
page-rs.overlays.default

# Add the below as a home-manager module
page-rs.nixosModules.default
```
- [Example for last step](https://github.com/DylanBulfin/nixos/blob/aff703b2df820fca7b82ea873a776cc75e220bc6/flake.nix#L32)
- Finally, add the following to your `home.nix`
```
programs.page-rs.enable = true;

# If you want to change default bindings
programs.page-rs.settings = 
{
    move_left = "m";
    move_down = "n";
    ...
};
```

## Configuration 
- Looks for a file at `~/.config/page-rs/config.toml`, path currently can't be changed
- Example of config below, with all values set to the default value:
```
move_left = "m"
move_down = "n"
move_up = "e"
move_right = "i"
exit = "q"
search = "/"
next_match = "k"
prev_match = "K"
```
- Should support any valid rust `char` assuming you have a keyboard to enter it
- Same options are supported for Nix modul settings

## TODO
- If called without stdin it will hang until you type some lines
- Can't be called on files currently
- Reads all of buffer in and keeps it in memory; probably fine for stdin but for file paging this will have to change

## Stretch Goals (probably not happening)
- Supports vim-style numeric prefixes for navigation commands
- Alternate mode that allows for textual selection and yanking to system clipboard
- Regex searching 