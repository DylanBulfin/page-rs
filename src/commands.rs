use std::{fs, path::Path};

use toml::{map::Map, Table, Value};

use crate::types::{Action, State};

pub(crate) struct Command {
    pub(crate) func: fn(&mut State) -> Action,
    key: char,
}

impl Command {
    pub fn new(key: char, func: fn(&mut State) -> Action) -> Self {
        Self { func, key }
    }

    pub(crate) fn key(&self) -> char {
        self.key
    }
}

fn get_key(name: &str, default: char, table: &Map<String, Value>) -> Result<char, ()> {
    let mut key = default;

    if table.contains_key(name) {
        if let Value::String(c) = &table[name] {
            key = c.chars().next().ok_or(())?;
        }
    }

    Ok(key)
}

pub fn initialize_commands(commands: &mut Vec<Command>) -> Result<(), ()> {
    let path = Path::new("~/.config/page-rs/config.toml");
    let mut table = Map::new();

    if path.exists() {
        table = fs::read_to_string(path)
            .expect("~/.config/page-rs/config.toml could not be read")
            .parse::<Table>()
            .expect("Unable to parse config.toml");
    }

    commands.push(Command::new(get_key("move_down", 'n', &table)?, move_down));
    commands.push(Command::new(get_key("move_up", 'e', &table)?, move_up));
    commands.push(Command::new(get_key("move_right", 'i', &table)?, move_right));
    commands.push(Command::new(get_key("move_left", 'm', &table)?, move_left));
    commands.push(Command::new(get_key("exit", 'q', &table)?, exit));
    commands.push(Command::new(get_key("search", '/', &table)?, search));
    commands.push(Command::new(get_key("next_match", 'k', &table)?, next_match));
    commands.push(Command::new(get_key("prev_match", 'K', &table)?, prev_match));
    
    Ok(())
}

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
