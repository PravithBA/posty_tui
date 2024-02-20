use core::panic;
use std::{error::Error, io};

use app::{Pane, State};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
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
            }

            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                return Ok(());
            }

            if key.modifiers == KeyModifiers::NONE {
                match state.selected_pane {
                    Pane::Index => {
                        let len_of_requests = state.requests.len();
                        if key.code == KeyCode::Char('c') {
                            state.requests.push("asdasd".into());
                        }
                        if let Some(selected_index) = state.index_list_state.selected() {
                            if key.code == KeyCode::Char('d') {
                                state.requests.remove(selected_index);
                                if state.requests.len() == selected_index {
                                    if selected_index != 0 {
                                        state.index_list_state.select(Some(selected_index - 1));
                                    } else {
                                        state.index_list_state.select(None)
                                    }
                                }
                            }
                            if key.code == KeyCode::Char('j') {
                                if selected_index >= len_of_requests - 1 {
                                    state.index_list_state.select(Some(0));
                                } else {
                                    state.index_list_state.select(Some(selected_index + 1));
                                }
                            }
                            if key.code == KeyCode::Char('k') {
                                if selected_index == 0 {
                                    state.index_list_state.select(Some(len_of_requests - 1));
                                } else {
                                    state.index_list_state.select(Some(selected_index - 1));
                                }
                            }
                        } else if !state.requests.is_empty() {
                            if key.code == KeyCode::Char('j') {
                                state.index_list_state.select(Some(0));
                            }
                            if key.code == KeyCode::Char('k') {
                                state.index_list_state.select(Some(len_of_requests - 1));
                            }
                        }
                    }
                    _ => {}
                }

                if key.code == KeyCode::Char('h') {
                    match state.selected_pane {
                        Pane::Index => {
                            state.selected_pane = Pane::ContentBody;
                        }
                        Pane::ContentUrl => {
                            state.selected_pane = Pane::Index;
                        }
                        Pane::ContentBody => {
                            state.selected_pane = Pane::ContentUrl;
                        }
                    }
                }

                if key.code == KeyCode::Char('i') {
                    state.selected_pane = Pane::Index;
                }

                if key.code == KeyCode::Char('b') {
                    state.selected_pane = Pane::ContentBody
                }

                if key.code == KeyCode::Char('u') {
                    state.selected_pane = Pane::ContentUrl;
                }

                if key.code == KeyCode::Char('l') {
                    match state.selected_pane {
                        Pane::Index => {
                            state.selected_pane = Pane::ContentUrl;
                        }
                        Pane::ContentUrl => {
                            state.selected_pane = Pane::ContentBody;
                        }
                        Pane::ContentBody => {
                            state.selected_pane = Pane::Index;
                        }
                    }
                }
            }
        }
    }
}
