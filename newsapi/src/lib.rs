use serde::Deserialize;
use url::Url;

const BASE_URL: &str = "https://newsapi.org/v2";

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Could not establish connection to newsapi")]
    RequestFailed(Box<ureq::Error>),
    #[error("Failed parsing respons to string")]
    FailedParseString(Box<std::io::Error>),
    #[error("Failed to create json from string")]
    FailedJsonFromString(Box<serde_json::Error>),
    #[error("Url parsing failed")]
    FailedParseUrl(#[from]url::ParseError)
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str, api_key: &str) -> Result<Articles, NewsApiError> {
    let res: String = ureq::get(url)
        .set("X-Api-Key", api_key)
        .call()
        .map_err(|e| NewsApiError::RequestFailed(Box::new(e)))?
        .into_string()
        .map_err(|e| NewsApiError::FailedParseString(Box::new(e)))?;
    
    let articles: Articles = serde_json::from_str(&res)
        .map_err(|e| NewsApiError::FailedJsonFromString(Box::new(e)))?;

    Ok(articles)
}

enum Endpoint {
    TopHeadlines,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string(),
        }
    }
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::Us => "us".to_string(),
            Self::De => "de".to_string(),
        }
    }
}

enum Country {
    Us,
    De,
}

struct NewsApi {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

impl NewsApi {
    fn new(api_key: &str) -> NewsApi {
        NewsApi {
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadlines,
            country: Country::De,
        }
    }

    fn country(&mut self, country: Country) -> &mut NewsApi {
        self.country = country;
        self
    }

    fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsApi {
        self.endpoint = endpoint;
        self
    }

    fn prepare_url(&self) -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut().unwrap().push(&self.endpoint.to_string());

        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));
        
        Ok(url.to_string())
    }
}
