use std::io::Result;

use crossterm::event::{Event, KeyEvent, KeyModifiers};

use crate::{commands, types::{Action, Pager}};

pub fn process_input(event: Event, pager: &mut Pager) -> Result<Action> {
    let redraw = match event {
        Event::Key(ke) => process_key_event(ke, pager)?,
        Event::Resize(w, h) => {
            pager.resize(w, h);
            Action::Redraw
        }
        _ => Action::None,
    };

    Ok(redraw)
}

fn process_key_event(event: KeyEvent, pager: &mut Pager) -> Result<Action> {
    match event.code {
        crossterm::event::KeyCode::Char(c) => process_char_key(c, event.modifiers, pager),
        _ => Ok(Action::None),
    }
}

fn process_char_key(c: char, mods: KeyModifiers, pager: &mut Pager) -> Result<Action> {
    if c == 'm' {
        Ok(commands::move_left(pager))
    } else if c == 'n' {
        Ok(commands::move_down(pager))
    } else if c == 'e' {
        Ok(commands::move_up(pager))
    } else if c == 'i' {
        Ok(commands::move_right(pager))
    } else if c == 'c' && mods == KeyModifiers::CONTROL {
        Ok(commands::exit(pager))
    } else {
        Ok(Action::None)
    }
}
