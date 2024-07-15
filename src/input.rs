use std::io::Result;

use crossterm::event::{Event, KeyEvent};

use crate::{commands, types::Pager};

fn process_key_event(event: KeyEvent, pager: &mut Pager) -> Result<bool> {
    match event.code {
        crossterm::event::KeyCode::Char(c) => process_letter_key(c, pager),
        _ => Ok(false),
    }
}

fn process_letter_key(c: char, pager: &mut Pager) -> Result<bool> {
    if c == 'm' {
        Ok(false)
    } else if c == 'n' {
        Ok(commands::move_down(pager))
    } else if c == 'e' {
        Ok(commands::move_up(pager))
    } else if c == 'i' {
        Ok(false)
    } else {
        Ok(false)
    }
}

pub fn process_input(event: Event, pager: &mut Pager) -> Result<bool> {
    let redraw = match event {
        Event::Key(ke) => process_key_event(ke, pager)?,
        Event::Resize(w, h) => {
            pager.resize(w, h);
            true
        }
        _ => false,
    };
    
    Ok(redraw)
}
