use crate::prelude::*;

pub struct Renderer {
    lines: usize,
    columns: usize,
    output: Vec<char>,
}

impl Renderer {
    pub fn new(columns: usize, lines: usize) -> Self {
        Self {
            columns,
            lines,
            output: Vec::with_capacity(columns * lines)
        }
    }

    pub fn push_str(&mut self, s: &str) {
        let mut chars: Vec<char> = s.chars().collect();
        self.output.append(&mut chars);
    }
}

impl io::Write for Renderer {
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.push_str(s);
                Ok(s.len())
            },
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", String::from_iter(self.output.clone()));
        stdout().flush()?;
        self.output.clear();
        out
    }
}
