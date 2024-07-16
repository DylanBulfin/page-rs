use crossterm::style::Stylize;

pub(crate) struct State {
    width: u16,
    height: u16,
    lnum: u16,
    cnum: u16,
    search_str: String,
    match_list: Vec<u16>,
    pub(crate) lines: Vec<String>,
}

impl State {
    pub fn new(width: u16, height: u16, lines: Vec<String>) -> Self {
        State {
            width,
            height,
            lnum: 0,
            cnum: 0,
            search_str: String::new(),
            match_list: Vec::new(),
            lines,
        }
    }

    pub fn move_to_line(&mut self, line: u16) -> u16 {
        if line >= self.height {
            self.lnum = self.height - 1;
        } else {
            self.lnum = line;
        }

        self.lnum
    }

    pub fn move_to_column(&mut self, col: u16) -> u16 {
        self.cnum = col;

        self.cnum
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.height = height;
        self.width = width;
    }

    pub fn set_search_term(&mut self, term: &str) -> bool {
        self.search_str = String::from(term);

        if term.len() == 0 {
            return false;
        }

        self.match_list = self
            .lines
            .iter()
            .enumerate()
            .filter(|(_, l)| l.contains(&self.search_str))
            .map(|(i, _)| i as u16)
            .collect();
        self.match_list.sort();
        return self.match_list.len() > 0;
    }

    pub fn get_search_term(&mut self) -> &str {
        &self.search_str
    }

    pub fn next_match(&mut self) -> bool {
        if self.match_list.len() == 0 {
            return false;
        }

        for &i in &self.match_list {
            if i > self.lnum {
                self.lnum = i;
                return true;
            }
        }

        //Unable to find a match later in file, go to first match in file
        self.lnum = self.match_list[0];
        true
    }

    pub fn curr_or_next_match(&mut self) -> bool {
        if self.match_list.contains(&self.lnum) {
            true
        } else {
            self.next_match()
        }
    }

    pub fn prev_match(&mut self) -> bool {
        if self.match_list.len() == 0 {
            return false;
        }

        for &i in self.match_list.iter().rev() {
            if i < self.lnum {
                self.lnum = i;
                return true;
            }
        }

        //Unable to find a match later in file, go to first match in file
        self.lnum = self.match_list[self.match_list.len() - 1];
        true
    }

    pub fn print_styled_line(&self, line: &str, lnum: u16) {
        if !self.match_list.contains(&lnum) {
            print!("{}",line)
        } else {
            let (p1, p2) = line.split_once(&self.search_str).unwrap();
            let styled_term = String::from(&self.search_str).on_cyan();
            print!("{}{}{}", p1, styled_term, p2)
        }
    }

    pub fn line(&self) -> u16 {
        self.lnum
    }

    pub fn column(&self) -> u16 {
        self.cnum
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}

pub enum Action {
    Redraw,
    Exit,
    None,
    Search,
    NextMatch,
    PrevMatch,
}

pub enum SearchAction {
    Write(char),
    Backspace,
    Cancel,
    Submit,
    None,
}
