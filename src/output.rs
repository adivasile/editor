use crate::prelude::*;

pub struct Frame {
    win_size: (usize, usize),
    editor_contents: EditorContents,
    cursor_controller: CursorController,
    active_buffer: Buffer,
}

impl Frame {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize - 1))
            .unwrap();

        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
            active_buffer: Buffer::new(),
        }
    }

    pub fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;

        if self.active_buffer.is_blank() {
            self.editor_contents.push_welcome_message(screen_columns, screen_rows);
            return
        }

        for i in 0..screen_rows {
            if let Some(buffer_line) = self.active_buffer.get_line_with_offset(i) {
                let render_line = format!(
                    "{:width$} {}",
                    buffer_line.line_number,
                    buffer_line.line,
                    width = GUTTER_WIDTH - 1
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
            self.cursor_controller.position.line,
            self.cursor_controller.position.column,
        );

        self.editor_contents.push_str(&info);

        for i in info.len()..self.win_size.0 {
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
        self.active_buffer.get_line_with_offset(self.cursor_controller.position.line)
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

    pub fn move_cursor(&mut self, direction: char) {
        match direction {
            'h' => self.cursor_controller.move_cursor_left(),
            'j' => {
                if self.cursor_controller.position.line >= self.win_size.1 - 1 {
                    self.active_buffer.scroll_down();
                } else {
                    self.cursor_controller.move_cursor_down();
                }
                self.snap_to_eol();
            },
            'k' => {
                if self.cursor_controller.position.line <= 0 {
                    self.active_buffer.scroll_up();
                } else {
                    self.cursor_controller.move_cursor_up();
                }

                self.snap_to_eol();
            },
            'l' => {
                if let Some(current_line) = self.current_buffer_line() {
                    if current_line.line.len() > 0 && self.cursor_controller.position.column < current_line.line.len() - 1 {
                        self.cursor_controller.move_cursor_right();
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
}
