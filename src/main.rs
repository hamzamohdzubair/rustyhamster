#[derive(serde::Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

// type Articles = Vec<Article>;

#[derive(serde::Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}


fn get_articles(url: &str) -> Result<Articles, Box<dyn std::error::Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;
    dbg!(articles);
    todo!()
    // Ok(response)
}

fn main() {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=a0998ee4103d47efbe9b1853f1006fd6";
    let articles = get_articles(url);
}