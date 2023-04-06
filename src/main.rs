use std::error::Error;

use newsapi::{Article, Country, Endpoint, NewsApi};
mod theme;

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# *Top Headlines*");
    theme.print_text("---\n\n");
    for a in articles {
        theme.print_text(&format!("`{}`\n", a.title));
        theme.print_text(&format!("*{}*", a.url));
        theme.print_text("---");
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let api_key = "f5fd5e5064bf44ae862e45f6a08c945a";

    let mut newsapi = NewsApi::new(api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::De);

    let newsapi_response = newsapi.fetch();

    render_articles(&newsapi_response?.articles());

    Ok(())
}
