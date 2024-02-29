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
