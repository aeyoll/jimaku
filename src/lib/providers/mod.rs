use crate::lib::episode::Episode;
use crate::lib::lang::Lang;
use crate::lib::subtitle::Subtitle;
use anyhow::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub trait HttpProvider {
    fn name(&self) -> &str;

    fn get_lang(&self, lang: Lang) -> Result<String, Error>;

    fn get_query(&self) -> Result<String, Error>;

    fn search_subtitle(&self, lang: Lang) -> Result<(Episode, Subtitle), Error>;

    fn download_subtitle(&self, subtitle: Subtitle) -> Result<String, Error>;

    fn write_subtitle(&self, contents: String) -> Result<(), Error>;
}

impl Display for dyn HttpProvider {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.name())
    }
}

pub struct Providers {
    pub providers: Vec<Box<dyn HttpProvider>>,
}

impl Providers {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn push<S: HttpProvider + 'static>(&mut self, provider: S) -> &mut Self {
        self.providers.push(Box::new(provider));

        self
    }

    pub fn run(&mut self, language: Lang) -> Result<(), Error> {
        for provider in self.providers.iter_mut() {
            info!("Searching using {}", provider);
            let subtitle = match provider.search_subtitle(language.clone()) {
                Ok((episode, subtitle)) => {
                    info!(
                        "Found subtitle for \"{}: {} ({})\"",
                        episode.show.title, episode.title, episode.code
                    );
                    subtitle
                }
                Err(_) => {
                    info!("No subtitle found for this episode");
                    continue;
                }
            };

            let contents = match provider.download_subtitle(subtitle) {
                Ok(contents) => {
                    info!("Fetched subtitle content");
                    contents
                }
                Err(_) => {
                    error!("Failed to download the subtitle");
                    continue;
                }
            };

            match provider.write_subtitle(contents) {
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    warn!("{}", e.to_string());
                }
            };
        }

        Ok(())
    }
}

pub mod betaseries;
pub mod opensubtitles;
