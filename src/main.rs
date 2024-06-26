use core::panic;
use std::{error::Error, io};

use crossterm::{
    event::{self as crossterm_event, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::key::{handle_key, ExitInstruction};
use models::state::State;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;

mod enums;
mod event;
mod models;
mod traits;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut state = State::new(vec![]);
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

        if let Event::Key(key) = crossterm_event::read()? {
            if key.kind == crossterm_event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            };
            if let ExitInstruction::Exit(exit_message) = handle_key(key, state) {
                eprintln!("{}", exit_message);
                return Ok(());
            };
        }
    }
}
