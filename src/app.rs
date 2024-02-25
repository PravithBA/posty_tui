use ratatui::widgets::ListState;

pub enum Pane {
    Index,
    ContentBody,
    ContentUrl,
    ContentMethod,
}

impl Pane {
    pub fn get_next_pane(&self) -> Pane {
        match self {
            Pane::Index => Pane::ContentMethod,
            Pane::ContentMethod => Pane::ContentUrl,
            Pane::ContentUrl => Pane::ContentBody,
            Pane::ContentBody => Pane::Index,
        }
    }

    pub fn get_prev_pane(&self) -> Pane {
        match self {
            Pane::Index => Pane::ContentBody,
            Pane::ContentMethod => Pane::Index,
            Pane::ContentUrl => Pane::ContentUrl,
            Pane::ContentBody => Pane::ContentUrl,
        }
    }
}

impl ToString for Pane {
    fn to_string(&self) -> String {
        match self {
            Pane::Index => "Index",
            Pane::ContentMethod => "Method",
            Pane::ContentUrl => "Url",
            Pane::ContentBody => "Body",
        }
        .into()
    }
}

pub enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl RequestMethod {
    pub fn get_list() -> Vec<RequestMethod> {
        vec![
            RequestMethod::Get,
            RequestMethod::Post,
            RequestMethod::Put,
            RequestMethod::Delete,
        ]
    }
}

impl ToString for RequestMethod {
    fn to_string(&self) -> String {
        match self {
            RequestMethod::Get => "Get",
            RequestMethod::Post => "Post",
            RequestMethod::Put => "Put",
            RequestMethod::Delete => "Delete",
        }
        .into()
    }
}

pub struct Request {
    pub label: String,
    pub method: RequestMethod,
    pub url: String,
}

impl Request {
    pub fn new(label: String) -> Request {
        Request {
            label,
            method: RequestMethod::Get,
            url: "".into(),
        }
    }
}

pub struct State {
    pub selected_pane: Pane,
    pub requests: Vec<Request>,
    pub index_list_state: ListState,
}

impl State {
    pub fn new() -> State {
        State {
            selected_pane: Pane::Index,
            requests: vec![],
            index_list_state: ListState::default().with_selected(None),
        }
    }

    pub fn move_to_next_pane(&mut self) {
        self.selected_pane = self.selected_pane.get_next_pane();
    }

    pub fn move_to_prev_pane(&mut self) {
        self.selected_pane = self.selected_pane.get_prev_pane();
    }
}
