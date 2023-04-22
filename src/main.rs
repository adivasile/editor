mod reader;
mod cleanup;
mod frame;
mod buffer;
mod editor_contents;
mod cursor_controller;
mod key_handler;

mod prelude {
    pub use std::time::Duration;
    pub use crossterm::event::*;
    pub use std::path::Path;
    pub use crossterm::{cursor, event, execute, queue, terminal, style};
    pub use crossterm::terminal::{*, ClearType, EnterAlternateScreen, LeaveAlternateScreen };
    pub use crossterm::event::*;
    pub use std::io::{stdout, self};
    pub use std::io::Write;
    pub use std::{cmp, env, fs};
    pub use std::path::PathBuf;


    pub const VERSION: &str = "0.0.1";
    pub const _CURSOR_MARGIN: usize = 5;
    pub const GUTTER_WIDTH: usize = 6;

    pub use crate::reader::*;
    pub use crate::cleanup::*;
    pub use crate::frame::*;
    pub use crate::buffer::*;
    pub use crate::editor_contents::*;
    pub use crate::cursor_controller::*;
    pub use crate::key_handler::*;
}

use prelude::*;


struct Editor {
    reader: Reader,
    frame: Frame,
}

impl Editor {
    fn new(file: Option<PathBuf>) -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize - 1))
            .unwrap();

        Self {
            reader: Reader,
            frame: Frame::new(
                win_size.0,
                win_size.1,
                file,
            ),
        }
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match KeyHandler::process_key(self.reader.read_key()?) {
            EditorCommand::QuitProgram => return Ok(false),
            EditorCommand::MoveCursorLeft => self.frame.move_cursor_left(),
            EditorCommand::MoveCursorRight => self.frame.move_cursor_right(),
            EditorCommand::MoveCursorUp => self.frame.move_cursor_up(),
            EditorCommand::MoveCursorDown => self.frame.move_cursor_down(),
            _ => {},
        }

        Ok(true)
    }

    fn run(&mut self) -> crossterm::Result<bool> {
        self.frame.refresh_screen()?;
        self.process_keypress()
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let file = match &args[..] {
        [_, arg] => Some(PathBuf::from(arg)),
        _ => None,
    };

    terminal::enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut editor = Editor::new(file);
    while editor.run()? {}

    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
