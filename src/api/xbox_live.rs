// src/api/xbox_live.rs

use crate::api::data::GamertagReservationResponse;
use crate::api::errors::GamertagError;
use reqwest::{Client, header::HeaderMap, StatusCode};
use std::env;

pub async fn reserve_gamertag(client: &Client, headers: HeaderMap) -> Result<GamertagReservationResponse, GamertagError> {
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
        .await
        .map_err(|e| GamertagError::RequestError(e.to_string()))?;

    match res.status() {
        StatusCode::OK => {
            let response_body: GamertagReservationResponse = res.json().await.map_err(|e| GamertagError::RequestError(e.to_string()))?;
            Ok(response_body)
        },
        status => {
            Err(GamertagError::ApiCallFailed { status: status.as_u16(), message: res.text().await.unwrap_or_else(|_| String::new()) })
        }
    }
}
