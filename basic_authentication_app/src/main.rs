use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    
    let client = Client::new();

    let user = "testUser".to_string();
    let passwd: Option<String> = None;

    let res=client.get("http://httpbin.org/").basic_auth(user,passwd).send();

    println!("response: {:?}",res);

    Ok(())
}
