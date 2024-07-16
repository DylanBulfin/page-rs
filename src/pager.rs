use crate::{input::process_input, types::Pager};
use std::{
    cmp::min,
    io::{stdout, Result, Write},
};

use crossterm::{cursor, event::read, queue, terminal};

pub fn draw_text(pager: &Pager) -> Result<()> {
    let mut stdout = stdout();
    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    for ln in pager.line().. {
        if ln >= pager.line() + pager.height() || ln >= pager.lines.len() as u16 {
            break;
        }

        let line = &pager.lines[ln as usize];
        let last_col = min(pager.column() + pager.width(), line.len() as u16);
        if pager.column() < last_col {
            let slice = &line[pager.column() as usize..last_col as usize];

            print!("{}", slice);
        }

        queue!(stdout, cursor::MoveToNextLine(1))?;
    }

    stdout.flush()
}

pub fn start(pager: &mut Pager) -> Result<()> {
    draw_text(pager)?;
    loop {
        match process_input(read()?, pager)? {
            crate::types::Action::Redraw => draw_text(pager)?,
            crate::types::Action::Exit => break,
            crate::types::Action::None => (),
        };
    }

    Ok(())
}
