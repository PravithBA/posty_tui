use ratatui::widgets::ListState;

pub enum Pane {
    Index,
    ContentBody,
    ContentUrl,
}

pub struct Request {
    pub label: String,
}

impl Request {
    pub fn new(label: String) -> Request {
        Request { label }
    }
}

pub struct State {
    pub selected_pane: Pane,
    pub url: String,
    pub requests: Vec<Request>,
    pub index_list_state: ListState,
}

impl State {
    pub fn new() -> State {
        State {
            selected_pane: Pane::Index,
            url: "".into(),
            requests: vec![],
            index_list_state: ListState::default().with_selected(Some(0)),
        }
    }
}
