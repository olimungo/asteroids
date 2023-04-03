pub enum Colors {
    Background,
    Edge,
    Dark,
}

impl Colors {
    pub fn value(&self) -> &str {
        match *self {
            Colors::Background => "#000000",
            Colors::Edge => "#dbe9ff",
            Colors::Dark => "#555555",
        }
    }
}
