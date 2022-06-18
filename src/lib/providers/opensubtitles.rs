use crate::lib::episode::Episode;
use crate::lib::providers::HttpProvider;
use crate::lib::show::Show;
use crate::lib::subtitle::Subtitle;
use crate::Lang;
use anyhow::Error;
use std::time::Duration;
use ureq::{Agent, Request};

const OPEN_SUBTITLES_API_KEY_HEADER: &str = "Api-Key";

#[derive(Clone, Copy)]
pub struct OpenSubtitlesLang {
    pub code: &'static str,
}

impl Lang for OpenSubtitlesLang {
    type Err = &'static str;

    fn get_lang(&self) -> &'static str {
        self.code
    }

    fn from_code(s: &str) -> Result<Self, Self::Err> {
        Ok(s)
    }
}

pub struct OpenSubtitleProvider {
    api_url: String,
    api_key: String,
}

impl OpenSubtitleProvider {
    pub fn new(api_key: String) -> Self {
        OpenSubtitleProvider {
            api_url: String::from("https://api.opensubtitles.com/api/v1/"),
            api_key,
        }
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
    fn search_subtitle<T: Lang>(
        &self,
        query: String,
        lang: &T,
    ) -> Result<(Episode, Subtitle), Error> {
        let language = lang.get_lang();
        let parameters = vec![("languages", language)];
        let qs = querystring::stringify(parameters);

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
        let subtitle: Subtitle = request.call()?.into_json()?;

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
}
