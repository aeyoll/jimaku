use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Show {
    pub id: i32,
    pub title: String,
}
