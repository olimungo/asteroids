pub enum Colors {
    Background,
    Edge,
    Warning,
    Dark,
    Light,
}

impl Colors {
    pub fn value(&self) -> &str {
        match *self {
            Colors::Background => "#000000",
            Colors::Edge => "#dbe9ff",
            Colors::Warning => "orange",
            Colors::Dark => "#555555",
            Colors::Light => "#999999",
        }
    }
}
