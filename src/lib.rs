pub mod attribute;
pub mod html;
pub mod model;

pub enum Msg {}

pub struct Location(String);

impl Location {
    pub fn new(s: impl Into<String>) -> Self {
        let str: String = s.into();
        Self(str.to_lowercase())
    }
}

impl Into<String> for Location {
    fn into(self) -> String {
        self.0
    }
}
