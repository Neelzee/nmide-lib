pub mod attribute;
pub mod html;
pub mod model;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    OnStart,
    /// All `PluginMsg`'s are in lowercase
    PluginMsg(String),
}

impl Msg {
    pub fn plugin_msg(s: impl Into<String>) -> Self {
        let msg: String = s.into();
        Self::PluginMsg(msg.to_lowercase())
    }
}

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
