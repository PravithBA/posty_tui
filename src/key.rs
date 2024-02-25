use crate::app::{Mode, Request};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{Pane, State};

pub fn handle_key(key: KeyEvent, state: &mut State) {
    match state.mode {
        Mode::Edit => handle_key_edit(key, state),
        Mode::Normal => handle_key_normal(key, state),
    }
}

fn handle_key_edit(key: KeyEvent, state: &mut State) {
    if key.code == KeyCode::Esc && key.modifiers == KeyModifiers::NONE {
        state.mode = Mode::Normal;
    }
    match state.selected_pane {
        Pane::ContentUrl => {
            if let Some(current_request) = state.get_current_request() {
                match key.code {
                    KeyCode::Char(key) => {
                        current_request.url = format!("{}{}", current_request.url, key);
                    }
                    KeyCode::Backspace => {
                        current_request.url.pop();
                    }
                    _ => {}
                };
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
    }
}

fn handle_key_normal(key: KeyEvent, state: &mut State) {
    if key.code == KeyCode::Char('i') && key.modifiers == KeyModifiers::NONE {
        state.mode = Mode::Edit;
    }
    if key.modifiers == KeyModifiers::NONE {
        match state.selected_pane {
            Pane::Index => {
                let len_of_requests = state.requests.len();
                if key.code == KeyCode::Char('c') {
                    let request = Request::new("".into());
                    state.requests.push(request);
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
}
