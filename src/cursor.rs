#![allow(non_snake_case)]
use std::vec;

use ratatui::prelude::*;
// --- End of declarations ------------------------------------------

#[derive(Clone)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    fn new() -> Cursor {
        Self { line: 0, column: 0 }
    }

    fn line(&self) -> usize {
        self.line.clone()
    }

    fn column(&self) -> usize {
        self.column.clone()
    }

    fn addCol(&mut self, n: usize) {
        self.column += n as usize;
    }

    fn subCol(&mut self, n: usize) {
        if self.column > (0 as usize) {
            self.column -= n;
        }
    }

    fn addLine(&mut self, l: usize) {
        self.line += l as usize;
    }
    fn subLine(&mut self, l: usize) {
        if self.line > 0 as usize {
            self.line -= l as usize;
        }
    }

    fn resetCol(&mut self) {
        self.column = 0;
    }
}

pub struct Editor {
    body: Vec<String>,
    cursor: Cursor,
}

impl Editor {
    pub fn new() -> Editor {
        Self {
            body: vec!["".to_string()],
            cursor: Cursor::new(),
        }
    }

    pub fn content<'a>(&'a self) -> Text<'a> {
        let x: Vec<Line> = self
            .body
            .clone()
            .into_iter()
            .map(|el| Line::from(el))
            .collect();

        Text::from(x)
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor.clone()
    }

    pub fn get_line_mut(&mut self) -> &mut String {
        &mut self.body[self.cursor.line()]
    }

    pub fn insert_char(&mut self, c: char) {
        let xPos = self
            .cursor
            .column
            .clone();
        let line = self.get_line_mut();

        line.insert(xPos, c);

        self.cursor
            .addCol(1);
    }

    pub fn enter(&mut self) {
        self.cursor
            .addLine(1);

        let length = self.body.len();
        let current_line = self.cursor.line() + 1;
        if length <= current_line {
            self.cursor
                .resetCol();
            self.body
                .push("".to_string());
        }
    }

    pub fn backspace(&mut self) {
        let xPos = self.cursor.column();

        if xPos == 0 {
            if self.cursor().line() > 0 {
                self.cursor.column = self.body[self.cursor.line() - 1].len();
                self.cursor.line -= 1;
                return;
            } else {
                return;
            }
        }

        self.cursor
            .subCol(1);

        let line = self.get_line_mut();
        line.remove(xPos - 1);
    }

    pub fn move_left(&mut self) {
        let xPos = self.cursor.column();

        if xPos == 0 {
            if self.cursor().line() > 0 {
                self.cursor.column = self.body[self.cursor.line() - 1].len();
                self.cursor.line -= 1;
                return;
            } else {
                return;
            }
        }

        self.cursor
            .subCol(1);
    }

    pub fn move_right(&mut self) {
        let xPos = self.cursor.column();
        let length = self.body[self.cursor.line].len();
        if xPos >= length {
            self.cursor
                .addLine(1);
            self.cursor
                .resetCol();

            // check if this is working
            match self
                .body
                .get(self.cursor.line())
            {
                Some(_) => return,
                None => {
                    self.body
                        .push("".to_string());
                    return;
                }
            }
        }

        self.cursor
            .addCol(1);
    }
}
