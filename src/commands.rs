use crate::types::{Action, Pager};

pub fn move_down(pager: &mut Pager) -> Action {
    pager.move_to_line(pager.line().saturating_add(1));
    Action::Redraw
}

pub fn move_up(pager: &mut Pager) -> Action {
    pager.move_to_line(pager.line().saturating_sub(1));
    Action::Redraw
}

pub fn move_right(pager: &mut Pager) -> Action {
    pager.move_to_column(pager.column().saturating_add(5));
    Action::Redraw
}

pub fn move_left(pager: &mut Pager) -> Action {
    pager.move_to_column(pager.column().saturating_sub(5));
    Action::Redraw
}

// Keeping uniformity in args because I want to make it generic one day
pub fn exit(_pager: &mut Pager) -> Action {
    Action::Exit
}
