use chlaty_core::{request_plugin::get_episode_list::DataResult};
use tabled::Tabled;

#[derive(Tabled)]
pub struct EpisodeListDisplay<'a> {
    pub index: usize,
    #[tabled(skip)]
    pub id: &'a str,
    pub title: &'a str
}