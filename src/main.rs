use std::error::Error;

use newsapi::{Articles, get_articles};
mod theme;

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# *Top Headlines*");
    theme.print_text("---\n\n");
    for a in &articles.articles {
        theme.print_text(&format!("`{}`\n", a.title));
        theme.print_text(&format!("*{}*", a.url));
        theme.print_text("---");
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let api_key = "f5fd5e5064bf44ae862e45f6a08c945a";
    let url = "https://newsapi.org/v2/top-headlines?country=de";

    let articles = get_articles(url, api_key)?;

    render_articles(&articles);

    Ok(())
}
