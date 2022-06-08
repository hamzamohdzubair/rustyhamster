#[cfg(feature = "async")]
use ureq::request;

const BASE_URL: &str = "https://newsapi.org/v3";

#[derive(thiserror::Error, Debug)]
pub enum STError {
    #[error("Failed fetching articles")]
    RequestFailed(#[from] ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(#[from] std::io::Error),
    #[error("Failed to parse articles")]
    ArticleParseFailed(serde_json::Error),
    #[error("Url parsing failed")]
    UrlParsing(#[from] url::ParseError),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
    #[error("Async request failed")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error)

}

#[derive(serde::Deserialize, Debug)]
pub struct STResponse {
    status: String,
    pub articles: Vec<Article>,
    code: Option<String>,
    // try type Articles = Vec<Article>;
}

impl STResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles

    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Article {
    title: String,
    url: String,
}

impl Article {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

// pub fn get_articles(url: &str) -> Result<Articles, STError> {
//     let response = ureq::get(url).call().map_err(|e| STError::RequestFailed(e))
//     ?.into_string().map_err(|e| STError::FailedResponseToString(e))?;
//     let articles = serde_json::from_str(&response).map_err(|e| STError::ArticleParseFailed(e))?;
//     // dbg!(articles);
//     Ok(articles)
// }

pub enum Endpoint {
    TopHeadlines,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string()
        }
    }
}

pub enum Country {
    Us
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::Us => "us".to_string()
        }
    }
}

pub struct SandyTurtle {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

impl SandyTurtle {
    pub fn new(api_key: &str) -> Self {
        Self { api_key: api_key.to_string(), endpoint: Endpoint::TopHeadlines, country: Country::Us }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;
        self
    }

    pub fn country(&mut self, country: Country) -> &mut Self {
        self.country = country;
        self
    }

    fn prepare_url(&self) -> Result<String, STError> {
        let mut url = url::Url::parse(BASE_URL)?;
        url.path_segments_mut().unwrap().push(&self.endpoint.to_string());
        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));
        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<STResponse, STError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("X-Api-Key", &self.api_key);
        let response: STResponse = req.call()?.into_json()?;
        match response.status.as_str() {
            "ok" => return Ok(response),
            _    => return Err(map_response_err(response.code))

        }
    }

    #[cfg(feature = "async")]
    pub async fn fetch_async(&self) -> Result<STResponse, STError> {
        let url = self.prepare_url()?;
        let client = reqwest::Client::new();
        let request = client.request(reqwest::Method::GET, url)
            .header("Authorization", &self.api_key)
            .build()
            .map_err(|e| STError::AsyncRequestFailed(e))?;
        
        let response: STResponse = client.execute(request).await?.json()
            .await.map_err(|e| STError::AsyncRequestFailed(e))?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _    => return Err(map_response_err(response.code))

        }

    }

}

fn map_response_err(code: Option<String>) -> STError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => STError::BadRequest("Your API key has been disabled"),
            _ => STError::BadRequest("Unknown error"),
        }
    }
    else {
        STError::BadRequest("Unknown error")
    }
}