use std::io::Result;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    commands,
    types::{Action, SearchAction, State},
};

pub fn process_input(event: Event, state: &mut State) -> Result<Action> {
    let action = match event {
        Event::Key(ke) => process_key_event(ke, state)?,
        Event::Resize(w, h) => {
            state.resize(w, h);
            Action::Redraw
        }
        _ => Action::None,
    };

    Ok(action)
}

fn process_key_event(event: KeyEvent, state: &mut State) -> Result<Action> {
    match event.code {
        KeyCode::Char(c) => process_char_key(c, event.modifiers, state),
        KeyCode::Esc => {
            if state.get_search_term().len() != 0 {
                state.set_search_term("");
                Ok(Action::Redraw)
            } else {
                Ok(Action::Exit)
            }
        }
        _ => Ok(Action::None),
    }
}

fn process_char_key(c: char, mods: KeyModifiers, state: &mut State) -> Result<Action> {
    if c == 'c' && mods == KeyModifiers::CONTROL {
        Ok(commands::exit(state))
    } else {
        match state.get_command(c) {
            Some(func) => Ok(func(state)),
            None => Ok(Action::None),
        }
    }
}

pub fn process_search_input(event: Event) -> Result<SearchAction> {
    let success = match event {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Char(c) => SearchAction::Write(c),
            KeyCode::Enter => SearchAction::Submit,
            KeyCode::Backspace => SearchAction::Backspace,
            KeyCode::Esc => SearchAction::Cancel,
            _ => SearchAction::None,
        },
        Event::Resize(_, _) => SearchAction::Cancel,
        _ => SearchAction::None,
    };

    Ok(success)
}
