mod reader;
mod cleanup;
mod output;
mod buffer;
mod editor_contents;
mod cursor_controller;

mod prelude {
    pub use std::time::Duration;
    pub use crossterm::event::*;
    pub use std::path::Path;
    pub use crossterm::{cursor, event, execute, queue, terminal};
    pub use crossterm::terminal::{*, ClearType};
    pub use crossterm::event::*;
    pub use std::io::{stdout, self};
    pub use std::io::Write;
    pub use std::{cmp, env, fs};


    pub const VERSION: &str = "0.0.1";
    pub const CURSOR_MARGIN: usize = 5;
    pub const GUTTER_WIDTH: usize = 4;

    pub use crate::reader::*;
    pub use crate::cleanup::*;
    pub use crate::output::*;
    pub use crate::buffer::*;
    pub use crate::editor_contents::*;
    pub use crate::cursor_controller::*;
}

use prelude::*;


struct Editor {
    reader: Reader,
    output: Frame,
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader,
            output: Frame::new(),
        }
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            KeyEvent {
                code: KeyCode::Char(val @ ('h' | 'j' | 'k' | 'l')),
                modifiers: event::KeyModifiers::NONE,
            } => self.output.move_cursor(val),
            _ => {},
        }

        Ok(true)
    }

    fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}



fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;

    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
