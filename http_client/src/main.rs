use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://192.168.20.14:8080/about").await?.json::<HashMap<String, String>>().await?;
    println!("{:#?}", resp);
    Ok(())
}