// Inspired by https://github.com/XAMPPRocky/Mammut/blob/master/src/lib.rs

extern crate dotenv;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Client;

#[derive(Debug)]
pub struct Heroku {
    client: Client,
    headers: HeaderMap,
}

impl Heroku {
    pub fn new(auth_token: String) -> Self {
        let heroku_client = Client::new();

        let mut headers = HeaderMap::new();
        let accept = HeaderValue::from_str("application/vnd.heroku+json; version=3");
        headers.insert(header::ACCEPT, accept.unwrap());

        let auth = HeaderValue::from_str(&format!("Bearer {}", auth_token));
        headers.insert(header::AUTHORIZATION, auth.unwrap());

        Heroku {
            client: heroku_client,
            headers: headers,
        }
    }
}

pub async fn run_command(heroku_client: Heroku) -> Result<(), Box<dyn std::error::Error>> {
    println!("heroku_client {:?}", heroku_client);

    let request = heroku_client
        .client
        .get("https://api.heroku.com/apps")
        .headers(heroku_client.headers.clone());

    println!("request {:?}", request);

    let response = request.send().await?;

    println!("response {:?}", response);

    Ok(())
}
