use crate::prelude::*;

#[derive(Debug)]
pub struct EditorContents {
    content: String,
}

impl EditorContents {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn push(&mut self, ch: char) {
        self.content.push(ch);
    }

    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }

    pub fn push_line(&mut self, string: &str) {
        self.content.push_str(string);
        self.push_str("\r\n");
    }

    pub fn push_welcome_message(&mut self, screen_columns: usize, screen_rows: usize) {
        let mut welcome = format!("Editor -- Version {}", VERSION);
        if welcome.len() > screen_columns {
            welcome.truncate(screen_columns)
        }

        let mut padding = (screen_columns - welcome.len()) / 2;
        if padding != 0 {
            self.push('~');
            padding -= 1;
        }

        (0..padding).for_each(|_| self.push(' '));
        for i in 0..screen_rows {
            if i == screen_rows / 3 {
                self.push_str(&welcome);
            } else {
                self.push('~');
            }
        }
    }
}

impl io::Write for EditorContents {
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            },
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}
