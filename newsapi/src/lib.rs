use serde::Deserialize;
use url::Url;

const BASE_URL: &str = "https://newsapi.org/v2";

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Could not establish connection to newsapi")]
    RequestFailed(#[from]ureq::Error),
    #[error("Failed parsing respons to string")]
    FailedParseString(#[from]std::io::Error),
    #[error("Failed to create json from string")]
    FailedJsonFromString(#[from]serde_json::Error),
    #[error("Url parsing failed")]
    FailedParseUrl(#[from]url::ParseError),
    #[error("Request failed : {0}")]
    BadRequest(&'static str),
}

#[derive(Deserialize, Debug)]
pub struct NewsApiResponse {
    status: String,
    pub articles: Vec<Article>,
    code: Option<String>,
}

impl NewsApiResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub enum Endpoint {
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

pub enum Country {
    Us,
    De,
}

pub struct NewsApi {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

impl NewsApi {
    pub fn new(api_key: &str) -> NewsApi {
        NewsApi {
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadlines,
            country: Country::De,
        }
    }

    pub fn country(&mut self, country: Country) -> &mut NewsApi {
        self.country = country;
        self
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsApi {
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

    pub fn fetch(&self) -> Result<NewsApiResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url)
            .set("X-Api-Key", &self.api_key);
        let response: NewsApiResponse = req.call()?.into_json()?;

        match response.status.as_str() {
            "ok" => Ok(response),
            _ => Err(map_response_err(response.code))
        }
    }
}

fn map_response_err(code: Option<String>) -> NewsApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your api key has been disabled"),
            _ => NewsApiError::BadRequest("Unknown Error"),
        }
    } else {
        NewsApiError::BadRequest("Unknown Error")
    }
}
