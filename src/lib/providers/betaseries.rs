use crate::lib::episode::Episode;
use crate::lib::lang::Lang;
use crate::lib::providers::HttpProvider;
use crate::lib::subtitle::Subtitle;
use crate::File;
use anyhow::{anyhow, Error};
use querystring;
use serde::Deserialize;
use std::env;
use std::time::Duration;
use ureq::{Agent, Request};

const BETA_SERIES_API_KEY_HEADER: &str = "X-BetaSeries-Key";

#[derive(Deserialize)]
struct BetaSeriesEpisodeScrapperResponse {
    episode: Episode,
    errors: Vec<String>,
}

pub struct BetaSeriesProvider {
    file: File,
    api_url: String,
    api_key: String,
}

impl BetaSeriesProvider {
    pub fn new(file: File) -> Result<Self, Error> {
        let api_key = match env::var("BETA_SERIES_API_KEY") {
            Ok(beta_series_api_key) => beta_series_api_key,
            Err(_) => {
                return Err(anyhow!(
                    "Please set a BETA_SERIES_API_KEY environment variable"
                ))
            }
        };

        Ok(BetaSeriesProvider {
            file,
            api_url: String::from("https://api.betaseries.com/"),
            api_key,
        })
    }

    fn get_agent(&self) -> Agent {
        ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build()
    }

    pub fn get(&self, url: String) -> Request {
        self.get_agent()
            .get(&url)
            .set(BETA_SERIES_API_KEY_HEADER, &self.api_key)
    }

    pub fn _post(&self, url: String) -> Request {
        self.get_agent()
            .post(&url)
            .set(BETA_SERIES_API_KEY_HEADER, &self.api_key)
    }
}

impl HttpProvider for BetaSeriesProvider {
    fn name(&self) -> &str {
        "BetaSeries"
    }

    fn get_lang(&self, lang: Lang) -> Result<String, Error> {
        match lang.code.as_str() {
            "en" => Ok(String::from("VO")),
            "fr" => Ok(String::from("VF")),
            _ => return Err(anyhow!("Impossible to find language code")),
        }
    }

    fn get_query(&self) -> Result<String, Error> {
        Ok(self.file.get_filename().to_string_lossy().to_string())
    }

    fn search_subtitle(&self, lang: Lang) -> Result<(Episode, Subtitle), Error> {
        let query = self.get_query()?;
        info!("Searching subtitle for file \"{}\"", &query);

        let qs = querystring::stringify(vec![("file", &query)]);
        let url = format!("{}episodes/scraper?{}", self.api_url, qs);
        let request = self.get(url);

        let response: BetaSeriesEpisodeScrapperResponse = request.call()?.into_json()?;
        let episode = response.episode;
        let subtitles: Vec<Subtitle> = episode.subtitles.clone();

        let language = self.get_lang(lang)?;

        let lang_filtered_subtitles: Vec<Subtitle> = subtitles
            .into_iter()
            .filter(|subtitle| subtitle.language == language)
            .collect();

        let subtitle = match lang_filtered_subtitles.is_empty() {
            true => return Err(anyhow!("No subtitle found for this episode")),
            false => lang_filtered_subtitles.first().unwrap().clone(),
        };

        Ok((episode, subtitle))
    }

    fn download_subtitle(&self, subtitle: Subtitle) -> Result<String, Error> {
        let request = self.get(subtitle.url);
        let content = request.call()?.into_string()?;

        Ok(content)
    }

    fn write_subtitle(&self, contents: String) -> Result<(), Error> {
        self.file.download(contents)?;

        Ok(())
    }
}
