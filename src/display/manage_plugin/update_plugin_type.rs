
use std::fmt;

pub struct PluginDisplay {
    pub id: String,
    pub title: String,
    pub version: String
}

impl fmt::Display for PluginDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | v{}", self.title, self.version)
    }
}