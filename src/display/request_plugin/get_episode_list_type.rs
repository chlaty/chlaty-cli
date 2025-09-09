use tabled::Tabled;

#[derive(Tabled)]
pub struct EpisodeListDisplay {
    pub index: usize,
    pub title: String
}