use ratatui::widgets::ListState;

pub enum Pane {
    Index,
    ContentBody,
    ContentUrl,
}

pub struct State {
    pub selected_pane: Pane,
    pub url: String,
    pub requests: Vec<String>,
    pub index_list_state: ListState,
}

impl State {
    pub fn new() -> State {
        State {
            selected_pane: Pane::Index,
            url: "".into(),
            requests: vec!["asdasdasd".into()],
            index_list_state: ListState::default().with_selected(Some(0)),
        }
    }
}
