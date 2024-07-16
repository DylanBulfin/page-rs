use crate::types::{Action, State};

pub fn move_down(state: &mut State) -> Action {
    state.move_to_line(state.line().saturating_add(1));
    Action::Redraw
}

pub fn move_up(state: &mut State) -> Action {
    state.move_to_line(state.line().saturating_sub(1));
    Action::Redraw
}

pub fn move_right(state: &mut State) -> Action {
    state.move_to_column(state.column().saturating_add(5));
    Action::Redraw
}

pub fn move_left(state: &mut State) -> Action {
    state.move_to_column(state.column().saturating_sub(5));
    Action::Redraw
}

// Keeping uniformity in args because I want to make it generic one day
pub fn exit(_state: &mut State) -> Action {
    Action::Exit
}

pub fn search(_state: &mut State) -> Action {
    Action::Search
}

pub fn next_match(state: &mut State) -> Action {
    state.next_match();
    Action::NextMatch
}

pub fn prev_match(state: &mut State) -> Action {
    state.prev_match();
    Action::PrevMatch
}