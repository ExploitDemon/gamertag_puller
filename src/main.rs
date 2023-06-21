// src/main.rs
mod api;
mod utils;

use crate::api::errors::GamertagError;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), GamertagError> {
    env_logger::init();
    let client = Client::builder().build().map_err(|e| GamertagError::RequestError(e.to_string()))?;

    let headers = utils::headers::get_headers();

    match api::xbox_live::reserve_gamertag(&client, headers).await {
        Ok(response) => {
            println!("promptForClassicGamertag: {}", response.prompt_for_classic_gamertag);
            println!("Gamertag: {}", response.gamertag);
            println!("Composed Gamertag: {}", response.composed_gamertag);
            println!("Classic Gamertag: {}", response.classic_gamertag);
            Ok(())
        },
        Err(e) => Err(e),
    }
}
