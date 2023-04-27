mod reader;
mod cleanup;
mod frame;
mod buffer;
mod editor_contents;
mod cursor_controller;
mod key_handler;
mod renderer;

mod prelude {
    pub use std::time::Duration;
    pub use crossterm::event::*;
    pub use std::path::Path;
    pub use crossterm::{cursor, event, execute, queue, terminal, style, Command};
    pub use crossterm::terminal::{*, ClearType, EnterAlternateScreen, LeaveAlternateScreen };
    pub use crossterm::event::*;
    pub use std::io::{stdout, self};
    pub use std::io::Write;
    pub use std::{cmp, env, fs, fmt};
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
    pub use crate::renderer::*;
}

use prelude::*;


pub enum Mode {
    Normal,
    Command,
}

struct Editor {
    reader: Reader,
    frames: Vec<Frame>,
    active_frame_idx: usize,
    mode: Mode,
    editor_contents: EditorContents,
    current_command: String,
    lines: usize,
    columns: usize,
}

impl Editor {
    fn new(file: Option<PathBuf>) -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize - 1))
            .unwrap();

        let file2 = file.clone();

        let frame = Frame::new(
            0,
            0,
            win_size.0 / 2,
            win_size.1 - 1,
            file
        );

        let frame2 = Frame::new(
            win_size.0 - frame.size.columns,
            0,
            win_size.0 - frame.size.columns,
            win_size.1 - 1,
            file2,
        );

        Self {
            editor_contents: EditorContents::new(),
            mode: Mode::Normal,
            reader: Reader,
            frames: vec![frame, frame2],
            active_frame_idx: 0,
            current_command: String::new(),
            columns: win_size.0,
            lines: win_size.1,
        }
    }

    fn active_frame(&mut self) -> &mut Frame {
        &mut self.frames[self.active_frame_idx]
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match KeyHandler::process_key(self.reader.read_key()?, &self.mode) {
            EditorCommand::QuitProgram => return Ok(false),
            EditorCommand::MoveCursorLeft => self.active_frame().move_cursor_left(),
            EditorCommand::MoveCursorRight => self.active_frame().move_cursor_right(),
            EditorCommand::MoveCursorUp => self.active_frame().move_cursor_up(),
            EditorCommand::MoveCursorDown => self.active_frame().move_cursor_down(),
            EditorCommand::SetCommandMode => self.mode = Mode::Command,
            EditorCommand::SetNormalMode => {
                self.mode = Mode::Normal;
                self.current_command = String::new();
            },
            EditorCommand::WriteCommand(c) => {
                self.current_command.push(c)
            },
            EditorCommand::DeleteCommandChar => {
                self.current_command.pop();
            },
            EditorCommand::ExecuteCommand => {
                match self.current_command.as_str() {
                    "quit" => {
                        return Ok(false)
                    },
                    "q" => {
                        return Ok(false)
                    },
                    _ => {},
                }
            },
            EditorCommand::JumpStartOfWord(Direction::Forward) => {
                self.active_frame().jump_to_start_of_word_forward()
            },
            EditorCommand::JumpStartOfWord(Direction::Backward) => {
                self.active_frame().jump_to_start_of_word_backward()
            },
            EditorCommand::JumpEndOfWord(Direction::Forward) => {
                self.active_frame().jump_to_end_of_word_forward()
            },
            EditorCommand::GoToRightFrame => {
                self.active_frame_idx = 1;
            },
            EditorCommand::GoToLeftFrame => {
                self.active_frame_idx = 0;
            },
            _ => {},
        }

        Ok(true)
    }

    fn draw_command_line(&mut self) {
        if let Mode::Command = self.mode {
            let cmd = format!(":{}", self.current_command);
            self.editor_contents.push_str(&cmd);
        }
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(
            self.editor_contents,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        // let frame_rows = self.active_frame().draw_rows()?;
        // let status_bar_buffer = self.active_frame().draw_status_bar();

        for frame in self.frames.iter() {
            queue!(
                self.editor_contents,
                frame.draw_rows()?,
                frame.draw_status_bar()?,
            )?;
        }

        self.draw_command_line();
        let (cursor_row, cursor_line) = match self.mode {
            Mode::Normal => {
                (
                    self.active_frame().cursor_controller.absolute_coords().0 + self.active_frame().size.start_column,
                    self.active_frame().cursor_controller.absolute_coords().1 + self.active_frame().size.start_line,
                )
            },
            Mode::Command => {
                (self.current_command.len() + 1, self.lines)
            }
        };
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_row as u16, cursor_line as u16),
            cursor::Show,
        )?;
        self.editor_contents.flush()
    }

    fn run(&mut self) -> crossterm::Result<bool> {
        self.refresh_screen()?;
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
