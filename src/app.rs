use ratatui::widgets::ListState;

pub enum Pane {
    Index,
    ContentBody,
    ContentUrl,
    ContentMethod,
}

impl Pane {
    pub fn get_next(&self) -> Pane {
        match self {
            Pane::Index => Pane::ContentMethod,
            Pane::ContentMethod => Pane::ContentUrl,
            Pane::ContentUrl => Pane::ContentBody,
            Pane::ContentBody => Pane::Index,
        }
    }

    pub fn get_prev(&self) -> Pane {
        match self {
            Pane::Index => Pane::ContentBody,
            Pane::ContentMethod => Pane::Index,
            Pane::ContentUrl => Pane::ContentMethod,
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
    pub fn get_next(&self) -> RequestMethod {
        match self {
            RequestMethod::Get => RequestMethod::Post,
            RequestMethod::Post => RequestMethod::Put,
            RequestMethod::Put => RequestMethod::Delete,
            RequestMethod::Delete => RequestMethod::Get,
        }
    }

    pub fn get_prev(&self) -> RequestMethod {
        match self {
            RequestMethod::Get => RequestMethod::Delete,
            RequestMethod::Post => RequestMethod::Get,
            RequestMethod::Put => RequestMethod::Post,
            RequestMethod::Delete => RequestMethod::Put,
        }
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

    pub fn select_to_next_method(&mut self) {
        self.method = self.method.get_next();
    }

    pub fn select_to_prev_method(&mut self) {
        self.method = self.method.get_prev();
    }
}

pub enum Mode {
    Edit,
    Normal,
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Edit => "Edit",
            Mode::Normal => "Normal",
        }
        .into()
    }
}

pub enum Popup {
    CreateRequest,
}

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
