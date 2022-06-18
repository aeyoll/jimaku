use crate::lib::episode::Episode;
use crate::lib::lang::Lang;
use crate::lib::providers::HttpProvider;
use crate::lib::subtitle::Subtitle;
use anyhow::{anyhow, Error};
use querystring;
use serde::Deserialize;
use std::env;
use std::time::Duration;
use ureq::{Agent, Request};

const BETA_SERIES_API_KEY_HEADER: &str = "X-BetaSeries-Key";

#[derive(Clone, Copy)]
pub struct BetaSeriesLang {
    pub code: &'static str,
}

impl Lang for BetaSeriesLang {
    type Err = &'static str;

    fn get_lang(&self) -> &'static str {
        self.code
    }

    fn from_code(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(BetaSeriesLang { code: "VO" }),
            "fr" => Ok(BetaSeriesLang { code: "VF" }),
            _ => Err("Unknown lang"),
        }
    }
}

#[derive(Deserialize)]
struct BetaSeriesEpisodeScrapperResponse {
    episode: Episode,
    errors: Vec<String>,
}

pub struct BetaSeriesProvider {
    api_url: String,
    api_key: String,
}

impl BetaSeriesProvider {
    pub fn new() -> Result<Self, Error> {
        let api_key = match env::var("BETA_SERIES_API_KEY") {
            Ok(beta_series_api_key) => beta_series_api_key,
            Err(_) => {
                return Err(anyhow!(
                    "Please set a BETA_SERIES_API_KEY environment variable"
                ))
            }
        };

        Ok(BetaSeriesProvider {
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
    fn search_subtitle<T: Lang>(
        &self,
        query: String,
        lang: &T,
    ) -> Result<(Episode, Subtitle), Error> {
        let qs = querystring::stringify(vec![("file", query.as_str())]);
        let url = format!("{}episodes/scraper?{}", self.api_url, qs);
        let request = self.get(url);

        let response: BetaSeriesEpisodeScrapperResponse = request.call()?.into_json()?;
        let episode = response.episode;
        let subtitles: Vec<Subtitle> = episode.subtitles.clone();

        let language = lang.get_lang();

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
}
