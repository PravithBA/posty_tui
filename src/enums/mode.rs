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
