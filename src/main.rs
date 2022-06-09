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
    Ok(articles)
    // dbg!(articles);
    // todo!()
    // Ok(response)
}

// fn main() {
//     let url = "https://ewsapi.org/v2/top-headlines?country=us&apiKey=a0998ee4103d47efbe9b1853f1006fd6";
//     let articles = get_articles(url).unwrap();
//     dbg!(articles);

// }

fn render_articles(articles: &Articles){
    for i in &articles.articles {
        colour::dark_green_ln!("> {}", i.title);
        colour::yellow_ln!("({})", i.url);
        println!();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=a0998ee4103d47efbe9b1853f1006fd6";
    let articles = get_articles(url)?;
    render_articles(&articles);
    // dbg!(articles);
    Ok(())
}