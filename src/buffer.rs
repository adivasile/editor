use crate::prelude::*;

// pub struct BufferIter {
//     curr: &str,
//     next: Option<&str>,
// }

// impl Iterator for BufferIter {
//     type Item = &str;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.curr = self.next;
//         self.next =
//     }
// }

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
    pub fn new() -> Self {
        let mut arg = env::args();

        match arg.nth(1) {
            None => {
                Self {
                    lines: Vec::new(),
                    file_path: None,
                    line_offset: 0,
                }
            },
            Some(file) => {
                Self {
                    lines: Self::read_file(file.as_ref()),
                    file_path: Some(file.into()),
                    line_offset: 0,
                }
            }
        }
    }

    // fn iter(&self) -> BufferIter {
    //     BufferIter {
    //         curr: &self.row_contents[self.row_offset],
    //         next: self.get_row(self.row_offset + 1),
    //     }
    // }

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
