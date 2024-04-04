use crate::{
    enums::{body_type::BodyTypeEnum, request_method::RequestMethod},
    traits::array_enum::ArrayEnum,
};

pub struct Request {
    pub label: String,
    pub method: RequestMethod,
    pub url: String,
    pub body_type: BodyTypeEnum,
}

impl Request {
    pub fn new(label: String) -> Request {
        Request {
            label,
            method: RequestMethod::Get,
            url: "".into(),
            body_type: BodyTypeEnum::None,
        }
    }

    pub fn select_to_next_method(&mut self) {
        self.method = self.method.get_next();
    }

    pub fn select_to_prev_method(&mut self) {
        self.method = self.method.get_prev();
    }

    pub fn select_to_next_body_type(&mut self) {
        self.body_type = self.body_type.get_next();
    }

    pub fn select_to_prev_body_type(&mut self) {
        self.body_type = self.body_type.get_prev();
    }
}
