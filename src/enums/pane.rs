use crate::traits::array_enum::ArrayEnum;

pub enum Pane {
    Index,
    ContentBody,
    ContentBodyType,
    ContentUrl,
    ContentMethod,
}

impl ArrayEnum for Pane {
    fn get_next(&self) -> Pane {
        match self {
            Pane::Index => Pane::ContentMethod,
            Pane::ContentMethod => Pane::ContentUrl,
            Pane::ContentUrl => Pane::ContentBodyType,
            Pane::ContentBodyType => Pane::ContentBody,
            Pane::ContentBody => Pane::Index,
        }
    }

    fn get_prev(&self) -> Pane {
        match self {
            Pane::Index => Pane::ContentBody,
            Pane::ContentMethod => Pane::Index,
            Pane::ContentUrl => Pane::ContentMethod,
            Pane::ContentBodyType => Pane::ContentUrl,
            Pane::ContentBody => Pane::ContentBodyType,
        }
    }
    fn get_array_form() -> Vec<Self> {
        vec![Pane::Index, Pane::ContentBody, Pane::ContentBodyType, Pane::ContentUrl, Pane::ContentUrl]
    }
}

impl ToString for Pane {
    fn to_string(&self) -> String {
        match self {
            Pane::Index => "Index",
            Pane::ContentMethod => "Method",
            Pane::ContentUrl => "Url",
            Pane::ContentBody => "Body",
            Pane::ContentBodyType => "Body Type",
        }
        .into()
    }
}
