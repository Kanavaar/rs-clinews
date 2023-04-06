use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Could not establish connection to newsapi")]
    RequestFailed(Box<ureq::Error>),
    #[error("Failed parsing respons to string")]
    FailedParseString(Box<std::io::Error>),
    #[error("Failed to create json from string")]
    FailedJsonFromString(Box<serde_json::Error>),
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
