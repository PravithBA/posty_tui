use core::panic;
use std::{error::Error, io};

use app::State;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use key::handle_key;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod key;
mod ui;
use crate::ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut state = State::new();
    match run_app(&mut terminal, &mut state) {
        Ok(()) => {
            println!("Exiting")
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut State) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            };
            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                return Ok(());
            }
            handle_key(key, state)
        }
    }
}
