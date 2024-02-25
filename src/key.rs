use crate::app::{Mode, Popup, Request};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{Pane, State};

pub enum ExitInstruction {
    Exit(String),
    NoExit,
}

pub fn handle_key(key: KeyEvent, state: &mut State) -> ExitInstruction {
    match state.mode {
        Mode::Edit => handle_key_edit(key, state),
        Mode::Normal => handle_key_normal(key, state),
    }
}

fn handle_key_edit(key: KeyEvent, state: &mut State) -> ExitInstruction {
    if key.code == KeyCode::Esc && key.modifiers == KeyModifiers::NONE {
        state.set_mode(Mode::Normal);
        return ExitInstruction::NoExit;
    }
    if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
        state.set_mode(Mode::Normal);
        return ExitInstruction::NoExit;
    }
    match state.selected_pane {
        Pane::ContentUrl => {
            if let Some(current_request) = state.get_current_request() {
                current_request.url = modify_text_for_key(&current_request.url, key);
            }
        }
        Pane::ContentMethod => {
            if key.modifiers == KeyModifiers::NONE {
                if let Some(current_request) = state.get_current_request() {
                    if key.code == KeyCode::Char('j') {
                        current_request.select_to_next_method();
                    }
                    if key.code == KeyCode::Char('k') {
                        current_request.select_to_prev_method();
                    }
                }
            }
        }
        _ => {}
    };
    ExitInstruction::NoExit
}

fn handle_key_normal(key: KeyEvent, state: &mut State) -> ExitInstruction {
    if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
        return ExitInstruction::Exit("Successfully exited".into());
    }

    if let Some(popup) = &state.popup {
        match popup {
            Popup::CreateRequest => {
                if let Some(current_request) = state.get_current_request() {
                    if key.code == KeyCode::Enter
                        && key.modifiers == KeyModifiers::NONE
                        && !current_request.label.is_empty()
                    {
                        state.close_popup();
                    } else if key.code == KeyCode::Esc && key.modifiers == KeyModifiers::NONE {
                        state.remove_selected_request();
                        state.close_popup();
                    } else {
                        current_request.label = modify_text_for_key(&current_request.label, key);
                    }
                }
            }
        }
        return ExitInstruction::NoExit;
    }

    if key.modifiers == KeyModifiers::NONE {
        match state.selected_pane {
            Pane::ContentUrl => {
                if key.code == KeyCode::Char('i') && key.modifiers == KeyModifiers::NONE {
                    state.set_mode(Mode::Edit);
                }
            }
            Pane::ContentMethod => {
                if key.code == KeyCode::Char('i') && key.modifiers == KeyModifiers::NONE {
                    state.set_mode(Mode::Edit);
                }
            }
            Pane::Index => {
                let len_of_requests = state.requests.len();
                if key.code == KeyCode::Char('c') {
                    state.set_popup(Popup::CreateRequest);
                    let request = Request::new("".into());
                    state.requests.push(request);
                    state
                        .index_list_state
                        .select(Some(state.requests.len() - 1));
                }
                if let Some(selected_index) = state.index_list_state.selected() {
                    if key.code == KeyCode::Char('d') {
                        state.remove_selected_request();
                    }
                    if key.code == KeyCode::Char('j') && selected_index < len_of_requests - 1 {
                        state.index_list_state.select(Some(selected_index + 1));
                    }
                    if key.code == KeyCode::Char('k') && selected_index != 0 {
                        state.index_list_state.select(Some(selected_index - 1));
                    }
                } else if !state.requests.is_empty() {
                    if key.code == KeyCode::Char('j') {
                        state.index_list_state.select(Some(0));
                    }
                    if key.code == KeyCode::Char('k') {
                        state.index_list_state.select(Some(0));
                    }
                }
            }
            _ => {}
        }

        if key.code == KeyCode::Char('h') {
            state.move_to_prev_pane();
        }

        if key.code == KeyCode::Char('l') {
            state.move_to_next_pane()
        }
    } else if key.modifiers == KeyModifiers::CONTROL {
        if let Some(selected_index) = state.index_list_state.selected() {
            let jump_number = 20;
            if key.code == KeyCode::Char('u') {
                if selected_index > jump_number {
                    state
                        .index_list_state
                        .select(Some(selected_index - jump_number))
                } else {
                    state.index_list_state.select(Some(0))
                }
            }
            if key.code == KeyCode::Char('d') {
                if selected_index + jump_number < state.requests.len() {
                    state
                        .index_list_state
                        .select(Some(selected_index + jump_number))
                } else {
                    state
                        .index_list_state
                        .select(Some(state.requests.len() - 1))
                }
            }
        }
    }
    ExitInstruction::NoExit
}

fn modify_text_for_key(string: &String, key: KeyEvent) -> String {
    let mut string = string.into();
    if key.modifiers == KeyModifiers::NONE {
        match key.code {
            KeyCode::Char(key) => {
                string = format!("{}{}", string, key);
            }
            KeyCode::Backspace => {
                string.pop();
            }
            _ => {}
        };
    }
    string
}
