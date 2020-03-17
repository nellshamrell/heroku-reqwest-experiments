#[derive(Debug)]
pub struct Heroku {
   auth_token: String,
}

impl Heroku {
    pub fn new(auth_token: String) -> Heroku {
        Heroku {
            auth_token
        }
    }
}

pub async fn run_command() -> Result<(), Box<dyn std::error::Error>> {
// curl -nX GET https://api.heroku.com/apps \
// -H "Accept: application/vnd.heroku+json; version=3"
    let res = reqwest::get("https://api.heroku.com")
        .await?
        .text()
        .await?;
   
    println!("response: {:?}", res);
    println!("");

    Ok(())
}