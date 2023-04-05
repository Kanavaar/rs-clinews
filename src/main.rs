use std::error::Error;

struct Articles {
    articles: Vec<Article>
}

struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str, api_key: &str) -> Result<Article, Box<dyn Error>> {
    let res: String = ureq::get(&url)
        .set("X-Api-Key", &api_key)
        .call()?
        .into_string()?;

    dbg!(res);

    // Ok(())
    todo!()
}

fn main() {
    let api_key = "f5fd5e5064bf44ae862e45f6a08c945a";
    let url = "https://newsapi.org/v2/top-headlines?country=de";

    let articles = get_articles(url, api_key);
}
