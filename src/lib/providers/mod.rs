use crate::lib::episode::Episode;
use crate::lib::lang::Lang;
use crate::lib::subtitle::Subtitle;
use anyhow::Error;

pub trait HttpProvider {
    fn get_query(&self) -> Result<String, Error>;

    fn search_subtitle<T: Lang>(&self, lang: &T) -> Result<(Episode, Subtitle), Error>;

    fn download_subtitle(&self, subtitle: Subtitle) -> Result<String, Error>;
}

pub mod betaseries;
// pub mod opensubtitles;
