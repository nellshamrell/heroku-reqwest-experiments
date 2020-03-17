mod heroku;
use heroku::Heroku;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let heroku_client = Heroku::new(dotenv::var("AUTH_TOKEN").unwrap());

    heroku::run_command(heroku_client).await?;
    Ok(())
}