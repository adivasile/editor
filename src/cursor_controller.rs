use crate::prelude::*;

pub struct CursorPosition {
    pub column: usize,
    pub line: usize,
}

pub struct CursorController {
    pub position: CursorPosition,
    pub frame_columns: usize,
    pub frame_lines: usize,
}

impl CursorController {
    pub fn new(win_size: (usize, usize)) -> Self {
        Self {
            position: CursorPosition { column: 0, line: 0 },
            frame_columns: win_size.0,
            frame_lines: win_size.1,
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.position.line > 0 {
            self.position.line -= 1;
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.position.line < self.frame_lines {
            self.position.line += 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.position.column < self.frame_columns {
            self.position.column += 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.position.column > 0 {
            self.position.column -= 1;
        }
    }

    pub fn move_cursor_to_column(&mut self, col: usize) {
        self.position.column = col;
    }

    pub fn absolute_coords(&self) -> (usize, usize) {
        (self.position.column + GUTTER_WIDTH, self.position.line)
    }

    pub fn reset_cursor(&mut self) {
        self.position.column = 0;
        self.position.line = 0;
    }
}
