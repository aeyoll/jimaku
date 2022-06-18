use crate::lib::episode::Episode;
use crate::lib::subtitle::Subtitle;
use anyhow::Error;

pub trait HttpProvider {
    fn search_subtitle(&self, query: String, lang: String) -> Result<(Episode, Subtitle), Error>;

    fn download_subtitle(&self, subtitle: Subtitle) -> Result<String, Error>;
}

pub mod betaseries;
pub mod opensubtitles;
