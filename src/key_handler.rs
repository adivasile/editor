use crate::prelude::*;

pub struct KeyHandler;

pub enum EditorCommand {
    QuitProgram,
    MoveCursorRight,
    MoveCursorLeft,
    MoveCursorUp,
    MoveCursorDown,
    Noop,
}

impl KeyHandler {
    pub fn process_key(key_event: KeyEvent) -> EditorCommand {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => EditorCommand::QuitProgram,
            KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::MoveCursorLeft,
            KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::MoveCursorDown,
            KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::MoveCursorUp,
            KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::MoveCursorRight,
            _ => EditorCommand::Noop,
        }
    }
}
