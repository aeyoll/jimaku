use crate::lib::show::Show;
use crate::lib::subtitle::Subtitle;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Episode {
    pub id: i32,
    pub title: String,
    pub season: i32,
    pub episode: i32,
    pub code: String,
    pub description: String,
    pub date: String,
    pub subtitles: Vec<Subtitle>,
    pub show: Show,
}
