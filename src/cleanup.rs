use crate::prelude::*;

pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not turn off raw mode");
        Frame::clear_screen().expect("Error");
    }
}
