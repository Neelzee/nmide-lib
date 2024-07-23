use crate::{html::Html, model::Model, Location, Msg};

use anyhow::{Context, Result};
use semver::{Version, VersionReq};
use url::Url;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PluginType {
    Manager,
    Worker,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifesto {
    pub author: String,
    pub name: String,
    pub plugin_type: PluginType,
    pub version: Version,
    pub dependencies: Vec<Dependency>,
    pub url: Url,
}

impl PartialOrd for Manifesto {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.plugin_type.cmp(&other.plugin_type))
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub url: Url,
    pub version_requirement: VersionReq,
}

impl Dependency {
    pub fn new(url: String, version_requirement: String) -> Result<Self> {
        Ok(Self {
            url: Url::parse(&url).context(format!("Failed parsing URL: `{url}`"))?,
            version_requirement: VersionReq::parse(&version_requirement)
                .context(format!("Failed parsing URL: `{version_requirement}`"))?,
        })
    }
}

pub type FnInit = unsafe extern "C" fn(Model) -> Model;
pub type FnUpdate = unsafe extern "C" fn(Msg, Model) -> Model;
pub type FnView = unsafe extern "C" fn(Msg, Model) -> *mut Vec<(Html, Location)>;
