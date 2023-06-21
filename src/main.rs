// src/main.rs
mod api;
mod utils;

use std::error::Error;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let client = Client::builder().build()?;

    let headers = utils::headers::get_headers();

    match api::xbox_live::reserve_gamertag(&client, headers).await {
        Ok(response) => {
            println!("promptForClassicGamertag: {}", response.prompt_for_classic_gamertag);
            println!("Gamertag: {}", response.gamertag);
            println!("Composed Gamertag: {}", response.composed_gamertag);
            println!("Classic Gamertag: {}", response.classic_gamertag);
        },
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}

