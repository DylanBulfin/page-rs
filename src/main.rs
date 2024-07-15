mod pager;
mod types;
mod input;
mod commands;
use std::io::{stdin, BufRead, Result};

use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use types::Pager;

fn read_stdin() -> Result<Vec<String>> {
    let stdin = stdin().lock();
    stdin.lines().collect()
}

fn main() ->Result<()>{
    let lines = read_stdin()?;

    enable_raw_mode()?;

    let (width, height) = terminal::size()?;
    let mut pager = Pager::new(width, height, lines);

    pager::start(&mut pager)?;

    disable_raw_mode()?;
        
    loop{}
}
