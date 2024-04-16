use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

#[derive(Deserialize,Debug)]
struct User{
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url =  format!("https://api.github.com/repos/{owner}/{repo}/stargazers",owner = "rust-lang-nursery", repo="rust-cookbook");
    println!("url: {}",url);

    let client = reqwest::Client::new();


    let res = client.get(&url).header(USER_AGENT,"rust web-api-client demo").send().await?;

    let users : Vec<User> = res.json().await?;

    println!("{:?}",users);
    Ok(())
}
