use tabled::Tabled;

#[derive(Tabled)]
pub struct InstalledPluginListDisplay {
    pub id: String,
    pub title: String,
    pub version: String
}