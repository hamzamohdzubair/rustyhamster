mod theme;


fn render_articles(articles: &Vec<sandyturtle::Article>) {
    let theme = theme::default();
    theme.print_text("# Top Headlines\n\n");
    for article in articles {
        // colour::dark_green_ln!("> {}", article.title);
        theme.print_text(&format!("`{}`", article.title()));
        theme.print_text(&format!("> *{}*", article.url()));
        theme.print_text("---")
        // colour::yellow_ln!("- {}\n", article.url);
    }

}


// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv();
    let api_key = std::env::var("API_KEY").expect("API_KEY missing");
    // let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    // let url = format!("{}{}", url, api_key);

    let mut sandyturtle = sandyturtle::SandyTurtle::new(&api_key);
    sandyturtle.endpoint(sandyturtle::Endpoint::TopHeadlines)
        .country(sandyturtle::Country::Us)
        ;

    let stresponse = sandyturtle.fetch()?;//.await?;

    // let articles = sandyturtle::get_articles(&url)?;
    // dbg!(articles);
    render_articles(&stresponse.articles());
    Ok(())
}
