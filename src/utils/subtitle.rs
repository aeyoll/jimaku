use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Subtitle {
    pub id: i32,
    pub language: String,
    pub source: String,
    pub quality: i32,
    pub file: String,
    pub url: String,
    pub date: String,
}
