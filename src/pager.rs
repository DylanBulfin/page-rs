use crate::{input::process_input, types::Pager};
use std::io::{stdout, Result, Write};

use crossterm::{cursor, event::read, queue, terminal};

pub fn draw_text(pager: &Pager) -> Result<()> {
    let mut stdout = stdout();
    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    for ln in pager.line().. {
        if ln >= pager.line() + pager.height() {
            break;
        }

        print!("{}", pager.lines[ln as usize]);
        queue!(stdout, cursor::MoveToNextLine(1))?;
    }

    stdout.flush()
}

pub fn start(pager: &mut Pager) -> Result<()> {
    loop {
        if process_input(read()?, pager)? {
            draw_text(pager)?;
        }
    }
}
