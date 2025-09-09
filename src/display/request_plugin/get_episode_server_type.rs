use tabled::Tabled;

#[derive(Tabled)]
pub struct EpisodeServerDisplay<'a> {
    pub index: usize,
    pub title: &'a str,
    pub id: &'a str,
}