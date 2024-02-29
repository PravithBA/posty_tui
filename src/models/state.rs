use ratatui::widgets::ListState;

use crate::enums::{mode::Mode, pane::Pane, popup::Popup};

use super::request::Request;

pub struct State {
    pub selected_pane: Pane,
    pub requests: Vec<Request>,
    pub index_list_state: ListState,
    pub mode: Mode,
    pub popup: Option<Popup>,
}

impl State {
    pub fn new(requests: Vec<Request>) -> State {
        State {
            selected_pane: Pane::Index,
            requests,
            index_list_state: ListState::default().with_selected(None),
            mode: Mode::Normal,
            popup: None,
        }
    }

    pub fn move_to_next_pane(&mut self) {
        self.selected_pane = self.selected_pane.get_next();
    }

    pub fn move_to_prev_pane(&mut self) {
        self.selected_pane = self.selected_pane.get_prev();
    }
    pub fn get_selected_request(&mut self) -> Option<&mut Request> {
        match self.index_list_state.selected() {
            Some(selected_index) => Some(&mut self.requests[selected_index]),
            None => None,
        }
    }

    pub fn set_popup(&mut self, popup: Popup) {
        self.popup = Some(popup);
    }

    pub fn close_popup(&mut self) {
        self.popup = None;
    }

    pub fn remove_selected_request(&mut self) {
        if let Some(selected_index) = self.index_list_state.selected() {
            self.remove_request(selected_index)
        }
    }
    pub fn remove_request(&mut self, request_index: usize) {
        self.requests.remove(request_index);
        if self.requests.len() == request_index {
            if request_index != 0 {
                self.index_list_state.select(Some(request_index - 1));
            } else {
                self.index_list_state.select(None)
            }
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}
