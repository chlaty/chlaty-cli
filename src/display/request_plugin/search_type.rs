use chlaty_core::{manage_plugin::get_installed_plugin_list::SourceInfo};
use std::fmt;

pub struct PluginDisplay<'a> {
    pub key: &'a String,
    pub info: &'a SourceInfo,
}

impl<'a> fmt::Display for PluginDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | v{}", self.info.title, self.info.version)
    }
}


pub struct SearchDisplay<'a> {
    pub id: &'a String,
    pub title: &'a String,
}

impl<'a> fmt::Display for SearchDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}