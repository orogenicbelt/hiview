use std::error;

pub mod navigation;
pub mod state;

use notatin::parser::Parser;

use crate::app::state::State;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub state: State,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(parser: Parser) -> Self {
        App {
            running: true,
            state: State::new(parser),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
