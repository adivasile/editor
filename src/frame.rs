use crate::prelude::*;

#[derive(Debug)]
pub struct FrameSize {
    pub columns: usize,
    pub lines: usize,
    pub gutter_width: usize,
    pub text_columns: usize,
    pub text_lines: usize,
}

impl FrameSize {
    fn new(columns: usize, lines: usize) -> Self {
        Self {
            columns,
            lines,
            gutter_width: GUTTER_WIDTH,
            text_columns: columns - GUTTER_WIDTH,
            text_lines: lines - 1,
        }
    }
}

pub struct Frame {
    size: FrameSize,
    editor_contents: EditorContents,
    cursor_controller: CursorController,
    active_buffer: Buffer,
    line_offset: usize,
    column_offset: usize,
}

impl Frame {
    pub fn new(columns: usize, lines: usize, file: Option<PathBuf>) -> Self {
        let size = FrameSize::new(columns, lines);
        Self {
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new((size.text_columns, size.text_lines)),
            active_buffer: Buffer::new(file),
            size,
            line_offset: 0,
            column_offset: 0,
        }
    }

    pub fn draw_rows(&mut self) {
        if self.active_buffer.is_blank() {
            self.editor_contents.push_welcome_message(self.size.text_columns, self.size.text_lines);
            self.editor_contents.push_str("\r\n");
            return
        }

        for i in 0..self.size.text_lines {
            if let Some(buffer_line) = self.active_buffer.get_line(i + self.line_offset) {
                let max_len = cmp::min(self.size.text_columns, buffer_line.line.len());


                let render_line = format!(
                    "{:width$} {}",
                    buffer_line.line_number,
                    &buffer_line.line[self.column_offset..max_len + self.column_offset],
                    width = self.size.gutter_width - 1,
                );
                self.editor_contents.push_str(&render_line);
            } else {
                self.editor_contents.push('~');
            }

            self.editor_contents.push_str("\r\n");
        }
    }

    fn draw_status_bar(&mut self) {
        self.editor_contents
            .push_str(&style::Attribute::Reverse.to_string());

        let filename = self.active_buffer.file_path.as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No name]");

        let info = format!(
            "{} -- {} lines  {}/{}",
            filename,
            self.active_buffer.number_of_lines(),
            self.cursor_controller.position.line + self.line_offset + 1,
            self.cursor_controller.position.column + 1,
        );

        self.editor_contents.push_str(&info);

        for _i in info.len()..self.size.columns {
            self.editor_contents.push(' ')
        }
        self.editor_contents
            .push_str(&style::Attribute::Reset.to_string());
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(GUTTER_WIDTH as u16, 0 as u16))
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(
            self.editor_contents,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;
        self.draw_rows();
        self.draw_status_bar();
        let (cursor_row, cursor_line) = self.cursor_controller.absolute_coords();
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_row as u16, cursor_line as u16),
            cursor::Show,
        )?;
        self.editor_contents.flush()
    }

    pub fn current_buffer_line(&self) -> Option<&BufferLine> {
        self.active_buffer.get_line(self.cursor_controller.position.line + self.line_offset)
    }

    pub fn snap_to_eol(&mut self) {
        if let Some(current_line) = self.current_buffer_line() {
            if current_line.line.len() == 0 {
                self.cursor_controller.move_cursor_to_column(0);
            } else if self.cursor_controller.position.column > current_line.line.len() - 1 {
                self.cursor_controller.move_cursor_to_column(current_line.line.len() - 1);
            }
        }
    }

    pub fn scroll_down(&mut self) {
        if self.cursor_controller.position.line >= self.size.text_lines - 1 {
            self.line_offset += 1;
        } else {
            self.cursor_controller.move_cursor_down();
        }
        self.snap_to_eol();
    }

    pub fn scroll_up(&mut self) {
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

    pub fn scroll_left(&mut self) {
        self.cursor_controller.move_cursor_left();

        if self.cursor_controller.position.column == 0 {
            if self.column_offset == 0 {
                return
            }

            self.column_offset -= 1;
        }
    }

    pub fn scroll_right(&mut self) {
        if let Some(current_line) = self.current_buffer_line() {
            if  current_line.line.len() > 0
                && self.cursor_controller.position.column < current_line.line.len() - 1 {
                    self.cursor_controller.move_cursor_right();
                    if self.cursor_controller.position.column == self.cursor_controller.frame_columns {
                        eprintln!("At the end {}, {}", self.cursor_controller.position.column, self.cursor_controller.frame_columns);
                        self.column_offset += 1;
                    }
                }
        }
    }

    pub fn move_cursor(&mut self, direction: char) {
        match direction {
            'h' => self.scroll_left(),
            'j' => self.scroll_down(),
            'k' => self.scroll_up(),
            'l' => self.scroll_right(),
            _ => unimplemented!(),
        }
    }
}
