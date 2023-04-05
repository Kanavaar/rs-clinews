use std::error::Error;
use serde::Deserialize;
use colour::{dark_green, yellow};

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str, api_key: &str) -> Result<Articles, Box<dyn Error>> {
    let res: String = ureq::get(&url)
        .set("X-Api-Key", &api_key)
        .call()?
        .into_string()?;
    
    let articles: Articles = serde_json::from_str(&res)?;

    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_green!("> {}\n", a.title);
        yellow!("{}\n\n", a.url);
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let api_key = "f5fd5e5064bf44ae862e45f6a08c945a";
    let url = "https://newsapi.org/v2/top-headlines?country=de";

    let articles = get_articles(url, api_key)?;

    render_articles(&articles);

    Ok(())
}
