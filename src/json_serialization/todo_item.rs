use serde::Deserialize;

#[derive(Deserialize)]
pub struct ToDOItem {
    pub title: String,
    pub status: String
}
