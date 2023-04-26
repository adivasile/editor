use crate::prelude::*;
use crate::Mode;

pub struct KeyHandler;

pub enum Direction {
    Forward,
    Backward,
}

pub enum EditorCommand {
    QuitProgram,
    MoveCursorRight,
    MoveCursorLeft,
    MoveCursorUp,
    MoveCursorDown,
    SetCommandMode,
    SetNormalMode,
    Noop,
    WriteCommand(char),
    DeleteCommandChar,
    ExecuteCommand,
    JumpStartOfWord(Direction),
    JumpEndOfWord(Direction),
}

impl KeyHandler {
    pub fn process_key(key_event: KeyEvent, mode: &Mode) -> EditorCommand {
        match mode {
            Mode::Normal => Self::process_normal_mode_key(key_event),
            Mode::Command => Self::process_command_mode_key(key_event),
        }
    }

    pub fn process_normal_mode_key(key_event: KeyEvent) -> EditorCommand {
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
            KeyEvent {
                code: KeyCode::Char(':'),
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::SetCommandMode,
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::JumpStartOfWord(Direction::Forward),
            KeyEvent {
                code: KeyCode::Char('b'),
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::JumpStartOfWord(Direction::Backward),
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::JumpEndOfWord(Direction::Forward),
            _ => EditorCommand::Noop,
        }
    }

    pub fn process_command_mode_key(key_event: KeyEvent) -> EditorCommand {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::SetNormalMode,
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::DeleteCommandChar,
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::ExecuteCommand,
            KeyEvent { 
                code: KeyCode::Char(c),
                modifiers: event::KeyModifiers::NONE,
            } => EditorCommand::WriteCommand(c),
            _ => EditorCommand::Noop,
        }
    }
}
