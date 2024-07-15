 pub(crate) struct Pager {
    width: u16,
    height: u16,
    lnum: u16,
    pub(crate) lines: Vec<String>,
}

impl Pager {
    pub fn new(width: u16, height: u16, lines: Vec<String>) -> Self {
        Pager {
            width,
            height,
            lnum: 0,
            lines
        }
    }

    pub fn move_to_line(&mut self, line: u16) -> u16 {
        let mut new = line;
        if line < 0 {
            new = 0;
        } else if line >= self.height {
            new = self.height - 1;
        } else {
            new = line;
        }

        self.lnum = new;

        new
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.height = height;
        self.width = width;
    }

    pub fn line(&self) -> u16 {
        self.lnum
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}
