use crate::{prelude::*, editor_contents};

#[derive(Debug)]
pub struct FrameSize {
    pub start_column: usize,
    pub start_line: usize,
    pub columns: usize,
    pub lines: usize,
    pub gutter_width: usize,
    pub text_columns: usize,
    pub text_lines: usize,
}

impl FrameSize {
    fn new(start_column: usize, start_line: usize, columns: usize, lines: usize) -> Self {
        Self {
            start_column,
            start_line,
            columns,
            lines,
            gutter_width: GUTTER_WIDTH,
            text_columns: columns - GUTTER_WIDTH - 1,
            text_lines: lines,
        }
    }
}

pub struct Frame {
    pub size: FrameSize,
    pub cursor_controller: CursorController,
    active_buffer: Buffer,
    line_offset: usize,
    column_offset: usize,
}

impl Frame {
    pub fn new(start_column: usize, start_line: usize ,columns: usize, lines: usize, file: Option<PathBuf>) -> Self {
        let size = FrameSize::new(start_column, start_line, columns, lines);
        Self {
            cursor_controller: CursorController::new((size.text_columns, size.text_lines)),
            active_buffer: Buffer::new(file),
            size,
            line_offset: 0,
            column_offset: 0,
        }
    }

    pub fn draw_rows(&self) -> crossterm::Result<EditorContents> {
        let mut editor_contents = EditorContents::new();

        for i in 0..self.size.text_lines {
            if let Some(buffer_line) = self.active_buffer.get_line(i + self.line_offset) {
                let line =  buffer_line.line_slice(self.column_offset, self.column_offset + self.size.text_columns);

                let render_line = format!(
                    "{:width$} {}",
                    buffer_line.line_number,
                    &line,
                    width = self.size.gutter_width - 1,
                );
                

                queue!(
                    editor_contents,
                    cursor::MoveTo((self.size.start_column) as u16, (self.size.start_line + i) as u16),
                    style::Print(&render_line),
                    cursor::SavePosition,
                    cursor::MoveTo((self.size.columns) as u16, i as u16),
                    style::SetAttribute(style::Attribute::Reverse),
                    style::Print(' '),
                    style::SetAttribute(style::Attribute::Reset),
                )?;

            } else {
                queue!(
                    editor_contents,
                    style::Print("~"),
                    cursor::MoveToNextLine(1),
                )?;
            }

        }

        Ok(editor_contents)
    }

    pub fn draw_status_bar(&self) -> crossterm::Result<EditorContents> {
        let mut editor_contents = EditorContents::new();

        editor_contents.push_str(&style::Attribute::Reverse.to_string());

        let filename = self.active_buffer.file_path.as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No name]");

        let render_line = format!(
            "{} -- {} lines  {}/{} -- {}/{}",
            filename,
            self.active_buffer.number_of_lines(),
            self.cursor_controller.position.line + self.line_offset + 1,
            self.cursor_controller.position.column + 1,
            self.cursor_controller.frame_columns,
            self.cursor_controller.frame_lines,
        );

        queue!( 
            editor_contents,
            cursor::MoveTo((self.size.start_column) as u16, (self.size.lines) as u16),
            style::Print(&render_line),
            style::Print(" ".repeat(self.size.columns - render_line.len())),
            style::SetAttribute(style::Attribute::Reset),
        )?;

        
        Ok(editor_contents)
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(GUTTER_WIDTH as u16, 0 as u16))
    }

    fn current_buffer_line(&self) -> Option<&BufferLine> {
        self.active_buffer.get_line(self.cursor_controller.position.line + self.line_offset)
    }

    fn snap_to_eol(&mut self) {
        if let Some(current_line) = self.current_buffer_line() {
            if current_line.line.len() == 0 {
                self.cursor_controller.move_cursor_to_column(0);
            } else if self.cursor_controller.position.column > current_line.line.len() - 1 {
                self.cursor_controller.move_cursor_to_column(current_line.line.len() - 1);
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_controller.position.line >= self.size.text_lines - 1 {
            self.line_offset += 1;
        } else {
            self.cursor_controller.move_cursor_down();
        }
        self.snap_to_eol();
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_controller.position.line <= 0 {
            if self.line_offset == 0 {
                return
            }
            self.line_offset -= 1;
        } else {
            self.cursor_controller.move_cursor_up();
        }
        self.snap_to_eol();
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_controller.position.column == 0 {
            if self.column_offset == 0 {
                return
            }

            self.column_offset -= 1;
        } else {
            self.cursor_controller.move_cursor_left();
        }
    }

    pub fn jump_to_start_of_word_forward(&mut self) {
        if let Some(current_line) = self.current_buffer_line() {
            if current_line.line.len() > 0 {
                let current_cursor_column = self.cursor_controller.position.column;
                let from_cursor = &current_line.line[current_cursor_column..];
                let jump_to_idx = from_cursor.find(' ');
                if let Some(idx) = jump_to_idx {
                    self.cursor_controller.position.column += idx + 1;
                }
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        if let Some(current_line) = self.current_buffer_line() {
            if  current_line.line.len() > 0
                && self.cursor_controller.position.column < current_line.line.len() - 1 {
                    if self.cursor_controller.position.column + 1 == self.cursor_controller.frame_columns {
                        eprintln!("Line len {}, Col offset {}", current_line.line.len(), self.column_offset);

                        if self.column_offset + self.cursor_controller.position.column < current_line.line.len() - 1 {
                            self.column_offset += 1;
                        }
                    } else {
                        self.cursor_controller.move_cursor_right();
                    }
                }
        }
    }

    pub fn jump_to_start_of_word_backward(&mut self) {
        if let Some(current_line) = self.current_buffer_line() {
            if current_line.line.len() > 0 {
                let current_cursor_column = self.cursor_controller.position.column;
                let until_cursor = &current_line.line[..current_cursor_column];
                let jump_to_idx = until_cursor.rfind(' ');
                if let Some(idx) = jump_to_idx {
                    self.cursor_controller.position.column = idx + 1;
                }
            }
        }
    }

    pub fn jump_to_end_of_word_forward(&mut self) {
        if let Some(current_line) = self.current_buffer_line() {
            if current_line.line.len() > 0 {
                let current_cursor_column = self.cursor_controller.position.column;
                let from_cursor = &current_line.line[current_cursor_column..];
                let jump_to_idx = from_cursor.find(' ');
                if let Some(idx) = jump_to_idx {
                    self.cursor_controller.position.column += idx - 1;
                }
            }
        }
    }
}
