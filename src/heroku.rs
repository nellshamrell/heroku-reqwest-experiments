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
    fn new(auth_token: String) -> Self {
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

pub async fn run_command() -> Result<(), Box<dyn std::error::Error>> {
    let heroku_client = Heroku::new(dotenv::var("AUTH_TOKEN").unwrap());
    println!("heroku_client {:?}", heroku_client);
    //    curl -X POST https://api.heroku.com/apps \
    //-H "Accept: application/vnd.heroku+json; version=3" \
    //-H "Authorization: Bearer $HEROKU_API_KEY"

    // curl -nX GET https://api.heroku.com/apps \
    // -H "Accept: application/vnd.heroku+json; version=3"
    //    let res = reqwest::get("https://api.heroku.com")
    //       .await?
    //      .text()
    //     .await?;

    let request = heroku_client
        .client
        .get("https://api.heroku.com/apps")
        .headers(heroku_client.headers.clone());

    println!("request {:?}", request);

    let response = request.send().await?;

    println!("response {:?}", response);

    Ok(())
}
