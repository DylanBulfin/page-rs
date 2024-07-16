mod commands;
mod input;
mod pager;
mod types;
use std::{
    io::{stdin, BufRead, Result},
    panic,
};

use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use types::Pager;

fn read_stdin() -> Result<Vec<String>> {
    let stdin = stdin().lock();
    stdin.lines().collect()
}

fn main() -> Result<()> {
    panic::set_hook(Box::new(|s| {
        println!("{}", s);
        disable_raw_mode().unwrap();
    }));
    let lines = read_stdin()?;

    enable_raw_mode()?;

    let (width, height) = terminal::size()?;
    let mut pager = Pager::new(width, height, lines);

    pager::start(&mut pager)?;

    disable_raw_mode()
}
