mod commands;
mod input;
mod pager;
mod types;
use std::{
    io::{stdin, BufRead, Result},
    panic,
};

use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use types::State;

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
    let mut state = State::new(width, height, lines);

    pager::start(&mut state)?;

    disable_raw_mode()
}
