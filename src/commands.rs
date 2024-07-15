use crate::{pager, types::Pager};

// bool in return indicates whether a redraw is necessary

pub fn move_down(pager: &mut Pager) -> bool {
    pager.move_to_line(pager.line().saturating_add(1));
    true
}

pub fn move_up(pager: &mut Pager) -> bool {
    pager.move_to_line(pager.line().saturating_sub(1));
    true
}