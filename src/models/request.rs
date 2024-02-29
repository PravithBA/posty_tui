use crate::enums::request_method::RequestMethod;

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
