use crate::prelude::*;


pub struct BufferLine {
    pub line_number: usize,
    pub line: String,
}

pub struct Buffer {
    lines: Vec<BufferLine>,
    pub file_path: Option<PathBuf>,
    pub line_offset: usize,
}

impl Buffer {
    pub fn new(file: Option<PathBuf>) -> Self {
        match file {
            None => {
                Self {
                    lines: Self::build_welcome_buffer(),
                    file_path: None,
                    line_offset: 0,
                }
            },
            Some(file) => {
                Self {
                    lines: Self::read_file(&file),
                    file_path: Some(file.into()),
                    line_offset: 0,
                }
            }
        }
    }

    pub fn read_file(file: &Path) -> Vec<BufferLine> {
        let file_contents = fs::read_to_string(file).expect("Unable to read file");
        let mut lines: Vec<BufferLine> = Vec::new();

        for (line_number, line) in file_contents.lines().enumerate() {
            lines.push(
                BufferLine {
                    line: line.to_string(),
                    line_number: line_number + 1,
                }
            );
        }

        lines
    }

    pub fn build_welcome_buffer() -> Vec<BufferLine> {
        let mut lines: Vec<BufferLine> = vec![];

        for i in 0..5 {
            lines.push(
                BufferLine {
                    line_number: i + 1,
                    line: String::from("~\r")
                }
            );
        }

        let welcome_line = BufferLine {
            line_number: 6,
            line: format!("~                 Editor -- Version {}", VERSION),
        };

        lines.push(welcome_line);

        for i in 6..10 {
            lines.push(BufferLine {
                line_number: i + 1,
                line: String::from("~\r")
            })
        }

        lines
    }

    pub fn number_of_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn get_line(&self, at: usize) -> Option<&BufferLine> {
        if at >= self.number_of_lines() {
            return None
        }
        Some(&self.lines[at])
    }

    pub fn get_line_with_offset(&self, at: usize) -> Option<&BufferLine> {
        if self.line_offset + at >= self.number_of_lines() {
            return None;
        }
        Some(&self.lines[at + self.line_offset])
    }

    pub fn is_blank(&self) -> bool {
        self.number_of_lines() == 0
    }

    pub fn scroll_down(&mut self) {
        self.line_offset += 1;
    }

    pub fn scroll_up(&mut self) {
        if self.line_offset == 0 {
            return
        }
        self.line_offset -= 1;
    }
}
