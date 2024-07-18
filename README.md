# page-rs
I've never managed to get `bat` or `less` to feel good, so this is my attempt at a simple pager in rust.

## Features
- Fully configurable keybindings (probably want to change defaults)
- Basic left/down/up/right navigation
- Vim-style searching

## Configuration
- Looks for a file at `~/.config/page-rs/config.toml`, path currently can't be changed
- Example of config below, with all values set to the default value:
```
move_left = "m";
move_down = "n";
move_up = "e";
move_right = "i";
exit = "q";
search = "/";
next_match = "k";
prev_match = "K";
```
- Should support any valid rust `char` assuming you have a keyboard to enter it

## TODO
- If called without stdin it will hang until you type some lines
- Can't be called on files currently
- Reads all of buffer in and keeps it in memory; probably fine for stdin but for file paging this will have to change

## Stretch Goals (probably not happening)
- Supports vim-style numeric prefixes for navigation commands
- Alternate mode that allows for textual selection and yanking to system clipboard
- Regex searching 