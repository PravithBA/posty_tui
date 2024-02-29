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
