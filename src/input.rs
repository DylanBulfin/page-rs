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
    if c == 'm' {
        Ok(commands::move_left(state))
    } else if c == 'n' {
        Ok(commands::move_down(state))
    } else if c == 'e' {
        Ok(commands::move_up(state))
    } else if c == 'i' {
        Ok(commands::move_right(state))
    } else if c == 'c' && mods == KeyModifiers::CONTROL {
        Ok(commands::exit(state))
    } else if c == '/' {
        Ok(commands::search(state))
    } else if c == 'k' {
        Ok(commands::next_match(state))
    } else if c == 'K' {
        Ok(commands::prev_match(state))
    } else {
        Ok(Action::None)
    }
}

pub fn process_search_input(event: Event, state: &mut State) -> Result<SearchAction> {
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
