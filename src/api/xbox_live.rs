// src/api/xbox_live.rs
use crate::api::data::GamertagReservationResponse;
use reqwest::{Client, header::HeaderMap};
use std::error::Error;

pub async fn reserve_gamertag(client: &Client, headers: HeaderMap) -> Result<GamertagReservationResponse, Box<dyn Error>> {
    let url = "https://gamertag.xboxlive.com/gamertags/reserve";

    let body = r#"{
        "gamertag":"hyzaerd",
        "reservationId":"2535421559576896",
        "targetGamertagFields":"gamertag"
    }"#;

    let res = client.post(url)
        .headers(headers)
        .body(body.to_string())
        .send()
        .await?;

    let response_body: GamertagReservationResponse = res.json().await?;


    Ok(response_body)
}
