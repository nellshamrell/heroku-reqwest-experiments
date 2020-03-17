// These examples are largely from https://docs.rs/reqwest/0.10.4/reqwest/index.html
use std::collections::HashMap;

mod heroku;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //    example_one().await?;
    //    example_two().await?;
    //    example_three().await?;
    //    example_four().await?;
    heroku::run_command().await?;
    Ok(())
}

async fn example_one() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", resp);
    println!("");

    Ok(())
}

async fn example_two() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);
    println!("");
    Ok(())
}

async fn example_three() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;

    println!("response = {:?}", res);
    println!("");
    Ok(())
}

async fn example_four() -> Result<(), Box<dyn std::error::Error>> {
    let params = [("foo", "bar"), ("baz", "quux")];
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .form(&params)
        .send()
        .await?;

    println!("response = {:?}", res);
    println!("");
    Ok(())
}
