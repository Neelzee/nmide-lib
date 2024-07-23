use nmlugin::PluginType;

pub mod attribute;
pub mod html;
pub mod model;
pub mod nmlugin;
mod tests;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    OnStart,
    OnLoad {
        pl_type: PluginType,
        name: String,
    },
    /// All `PluginMsg`'s are in lowercase
    PluginMsg(String),
}

impl Msg {
    pub fn plugin_msg(s: impl Into<String>) -> Self {
        let msg: String = s.into();
        Self::PluginMsg(msg.to_lowercase())
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location(String);

impl Location {
    pub fn new(s: impl Into<String>) -> Self {
        let str: String = s.into();
        Self(str.to_lowercase())
    }

    pub fn to_string(self) -> String {
        self.0
    }
}

impl Into<String> for Location {
    fn into(self) -> String {
        self.0
    }
}
