
use std::fmt;

pub struct PluginDisplay {
    pub id: String,
    pub title: String,
    pub manifest: String,
}

impl fmt::Display for PluginDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {} ({})", self.title, "latest", self.id)
    }
}