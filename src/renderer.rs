use std::io::{stdout, Result, Stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub struct Renderer {}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) -> Result<Terminal<CrosstermBackend<Stdout>>> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        Ok(terminal)
    }

    pub fn cleanup(&self) -> Result<()> {
        stdout().execute(crossterm::terminal::LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}
