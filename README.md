# page-rs
I've yet to find a pager that fits my specific wants (I use Colemak-DH so I want `mnei` as basic nav keys, and normal vim bindings for basically everything else)

## Features
- Read from stdin, allow simple navigation via `mnei`
- Supports vim-style `/` searching, `kK` to go between matches (since `n` is taken)

## TODO
- If called without stdin it will hang until you type some lines
- Can't be called on files currently
- Reads all of buffer in and keeps it in memory; probably fine for stdin but for file paging this will have to change

## Stretch Goals (probably not happening)
- Supports vim-style numeric prefixes for navigation commands
- Alternate mode that allows for textual selection and yanking to system clipboard
- Regex searching 