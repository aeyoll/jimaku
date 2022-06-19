use crate::lib::episode::Episode;
use crate::lib::providers::HttpProvider;
use crate::lib::show::Show;
use crate::lib::subtitle::Subtitle;
use crate::{File, Lang};
use anyhow::{anyhow, Error};
use std::env;
use std::time::Duration;
use ureq::{Agent, Request};

const OPEN_SUBTITLES_API_KEY_HEADER: &str = "Api-Key";

pub struct OpenSubtitleProvider {
    file: File,
    api_url: String,
    api_key: String,
}

impl OpenSubtitleProvider {
    pub fn new(file: File) -> Result<Self, Error> {
        let api_key = match env::var("OPEN_SUBTITLES_API_KEY") {
            Ok(beta_series_api_key) => beta_series_api_key,
            Err(_) => {
                return Err(anyhow!(
                    "Please set a OPEN_SUBTITLES_API_KEY environment variable"
                ))
            }
        };

        Ok(OpenSubtitleProvider {
            file,
            api_url: String::from("https://api.opensubtitles.com/api/v1/"),
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
            .set(OPEN_SUBTITLES_API_KEY_HEADER, &self.api_key)
    }

    pub fn post(&self, url: String) -> Request {
        self.get_agent()
            .post(&url)
            .set(OPEN_SUBTITLES_API_KEY_HEADER, &self.api_key)
    }
}

impl HttpProvider for OpenSubtitleProvider {
    fn name(&self) -> &str {
        "OpenSubtitles"
    }

    fn get_lang(&self, lang: Lang) -> Result<String, Error> {
        Ok(lang.code)
    }

    fn get_query(&self) -> Result<String, Error> {
        let (hash, _) = self.file.get_hash();
        Ok(hash)
    }

    fn search_subtitle(&self, lang: Lang) -> Result<(Episode, Subtitle), Error> {
        let language = self.get_lang(lang)?;
        let qs = querystring::stringify(vec![("languages", language.as_ref())]);

        let url = format!("{}subtitles?{}", self.api_url, qs);
        let request = self.get(url);
        let response = request.call()?.into_string()?;
        println!("{:?}", response);

        let episode = Episode {
            id: 0,
            title: "".to_string(),
            season: 0,
            episode: 0,
            code: "".to_string(),
            description: "".to_string(),
            date: "".to_string(),
            subtitles: vec![],
            show: Show {
                id: 0,
                title: "".to_string(),
            },
        };
        let subtitle = Subtitle {
            id: 0,
            language,
            source: "".to_string(),
            quality: 0,
            file: "".to_string(),
            url: "".to_string(),
            date: "".to_string(),
        };

        Ok((episode, subtitle))
    }

    fn download_subtitle(&self, subtitle: Subtitle) -> Result<String, Error> {
        let url = format!("{}download", self.api_url);
        let data = ureq::json!({
            "file_id": subtitle.id
        });

        let request = self.post(url);
        let _response = request.send_json(data)?;

        Ok(String::from("TODO"))
    }

    fn write_subtitle(&self, contents: String) -> Result<(), Error> {
        self.file.download(contents)?;

        Ok(())
    }
}
