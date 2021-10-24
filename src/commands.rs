//! # Commands
//! This module contains enums and a function for converting crossterm
//! [KeyEvent](crossterm::event::KeyEvent)s into commands

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::sync::mpsc;

/// Enum indicating the type of event
pub enum Event {
    /// Event when a key has been pressed
    KeyInput(KeyEvent),
    /// Event when nothing has happened
    Tick,
}

/// Commands to be carried out
pub enum Command<'a> {
    /// Do nothing
    None,
    /// Quit and print the given message
    QuitWithError(&'a str),
    /// Quit with no message
    Quit,
}

/// Convert key inputs into `Command`s
///
/// # Arguments
///
/// * `rx` - An mpsc receiver which receives an [Event]
///
/// # Returns
///
/// The [Command] associated with the event
pub fn process_command_event(rx: &mpsc::Receiver<Event>) -> Command {
    match rx.try_recv() {
        Ok(e) => match e {
            Event::KeyInput(event) => match (event.code, event.modifiers) {
                (KeyCode::Char('q'), m) if m.is_empty() => Command::Quit,
                (KeyCode::Char('c'), m) if m.contains(KeyModifiers::CONTROL) => Command::Quit,
                _ => Command::None,
            },
            Event::Tick => Command::None,
        },
        Err(err) => {
            if err == mpsc::TryRecvError::Disconnected {
                Command::QuitWithError("mpsc channel disconnected")
            } else {
                Command::None
            }
        }
    }
}
