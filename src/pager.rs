use crate::{
    input::{process_input, process_search_input},
    types::{Action, State},
};
use std::{
    cmp::min,
    io::{stdout, Result, Write},
};

use crossterm::{cursor, event::read, queue, terminal};

pub fn draw_text(state: &State) -> Result<()> {
    let mut stdout = stdout();
    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    for ln in state.line().. {
        if ln >= state.line() + state.height() || ln >= state.lines.len() as u16 {
            break;
        }

        let line = &state.lines[ln as usize];
        let last_col = min(state.column() + state.width(), line.len() as u16);
        if state.column() < last_col {
            let slice = &line[state.column() as usize..last_col as usize];
            
            state.print_styled_line(slice, ln);
        }

        queue!(stdout, cursor::MoveToNextLine(1))?;
    }

    stdout.flush()
}

pub fn start(state: &mut State) -> Result<()> {
    draw_text(state)?;
    loop {
        match process_input(read()?, state)? {
            Action::Redraw => draw_text(state)?,
            Action::NextMatch => draw_text(state)?,
            Action::PrevMatch => draw_text(state)?,
            Action::Exit => break,
            Action::Search => {
                search_mode(state)?;
                draw_text(state)?;
            }
            Action::None => (),
        };
    }

    Ok(())
}

fn search_mode(state: &mut State) -> Result<()> {
    let mut stdout = stdout();

    queue!(
        stdout,
        cursor::MoveTo(0, state.height() - 1),
        terminal::Clear(terminal::ClearType::CurrentLine)
    )?;
    print!("/");

    stdout.flush()?;

    let mut search_term = String::new();

    loop {
        match process_search_input(read()?, state)? {
            crate::types::SearchAction::Write(c) => {
                search_term.push(c);
                print!("{}", c);
                stdout.flush()?
            }
            crate::types::SearchAction::Backspace => {
                if search_term.len() > 0 {
                    search_term.remove(search_term.len() - 1);
                    queue!(
                        stdout,
                        cursor::MoveLeft(1),
                        terminal::Clear(terminal::ClearType::UntilNewLine)
                    )?;
                    stdout.flush()?;
                }
                ()
            }
            crate::types::SearchAction::Cancel => return Ok(()),
            crate::types::SearchAction::Submit => {
                queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;
                print!("{}", search_term);
                stdout.flush()?;
                if state.set_search_term(&search_term) {
                    state.curr_or_next_match();
                }
                return Ok(());
            }
            crate::types::SearchAction::None => (),
        };
    }
}
