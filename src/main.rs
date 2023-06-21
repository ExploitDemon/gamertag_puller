// use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT, AUTHORIZATION};
// use reqwest::Proxy;
// use std::error::Error;
// use std::fs::File;
// use std::io::Read;
// use tokio;
// use serde::Deserialize;
//
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct GamertagReservationResponse {
//     prompt_for_classic_gamertag: bool,
//     classic_translation_level: String,
//     gamertag_suffix: String,
//     gamertag: String,
//     composed_gamertag: String,
//     classic_gamertag: String,
// }
//
//
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // URL
//     let url = "https://gamertag.xboxlive.com/gamertags/reserve";
//
//     // Headers
//     let mut headers = HeaderMap::new();
//     headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.5359.95 Safari/537.36"));
//     headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
//     headers.insert(AUTHORIZATION, HeaderValue::from_static(" XBL3.0 x=14887652135981289059;eyJlbmMiOiJBMTI4Q0JDK0hTMjU2IiwiYWxnIjoiUlNBLU9BRVAiLCJjdHkiOiJKV1QiLCJ6aXAiOiJERUYiLCJ4NXQiOiIxZlVBejExYmtpWklFaE5KSVZnSDFTdTVzX2cifQ.I04Eog8IYc6nAFOd10Td207oziWUvtQIDVNvSCS_JzcXSQawrbkg52sccWSLnNC_qHkXCYoSmDF8n6uZNBujFE7Oeirlzh_7gQDtZKVrtY8IIBaYN70z2E58yjJjbeXuA1DbdAa5Wk5W5mh7Gu_lBrSXJAOExp0ekgh4ebtfSOs.23ZrV1KDGwNDlHpgi6jrWA.bkjozqYGVs78D9QozO6ztTplFmFpi3_2FCIYHVLpowYMXCjdcruXue5edQ4sKPUWK5zhzfqgPiybRPrlDtkLGsl44zT-F-K96aRqogLxzCGtKgbJDJAbCvrRtghmUzT2fdqouaFyafPdAwMXRHwQsa6V_6Z001K79cALUWph0Ujhi2Ww---4qRcNedcsk_ZBMqoeF5q-zqgjlS3Rpg4p0tCO7Op1jkDxzCahSlmSqoF3Ba2DgIKOOGZwxwLbpjmtbcopcq_SQq2gJ5r-NWo_5hlL7Y3RZejYb6vwMIqTQXxt-SB44XZEQl9pjVOaqY6cFTO5i_5kcpR1AwKmQh0LvWIfZzjOEctapAtt0Pms5ja1Ry_EPSkDJZVuzCqsdjnt2RuN5UpEXGUSGGPMfD83aL9rmFGoHADqS-na0QkZPw4r3W9XB_v23zbv0TO38q8YIvUG0y_sk2BGjuz2QJ5DeGRX6pAzetg-Bg1j5oiGswW3wKuVSQs8irQ_RUDKX57LqcL5JFnQbOIOGIo5B7RtdJAAso50y_POWE4wwSrB354ih8L66sE0lsJCziWtuhMKwthzEfbu-O0sBwp1qFu8zpdEz6ki0TFfcI7WMPsaqr6Qx_WKZ4tc3CENOq8fv9ox24ZqWlU_Y7EYCazqNPvQskF7YCoxEjKzhu5acqPB0o9_8LqOE2bRsrU-DyhBMdsMOrut0LbNue0s5Zjyvi31gnJWt8llyqm-lDhO2rh6MUjLfJbT3Lt502B9N3s3okgFyEHl9mGPzTYYfi7S_2ahq-l8VJ-UzUWgEHBpIlLXR819ornaD5-iui5wRihX3-bG_azyufp2bDaRfl348PbQcFb1QGQxP_p2Kp9FYUYA4veHmfCDXigIueSxe5w1kuzmqhkhFtdRVZCDeRHqxFNqtRPVGRXmwCVjGM3Cr1KJLL42_hYVeAq3sRHFJQVRzRcwpXI7fLVNDDi1flGhQsEaf7puQGamhSgYrRkfRFtBC7JOtR1XQwL6LOX6wX9aNuMeA5eHZR4cVUeK6RbFhq_aR9D2-l656of_upwkRToJ2dexDV1EV8jY3ISoPXWaC9blChpGBdgWLAuGZas2zOR2vE-VXPT_O44doq6ieuuNfw_PqzoC_7MnSMICS2Q2ueK5pI37CNEWtGvRbTT__S_n6sqSNlNQc1k5mlybFkZVW1THjRL_Z3-iMkguojlpstkr1zi9SIyNF1fbSymICYlT5t5JvP8ihMeuX12I0tjKjCVqT1I4Hu5poQvnKlUUoVnx1P67suoIePsYh6oLNBJ1IizuCb9u1DLHXpqvzDqolv4W5EKR3271wx6vxjU7eKq0_RV3YXD70H0-KJxP6gChEyjeKbeE_fqtvCo3N72cIGntc6MTkH73eWgnQVwpnTRl0-WVvs44C197LqB9G-IAkwgQZfO8WWJZjmOQKvqFaG9fbjeIY3usSMBsqQW7XspAGReono5qqT5awwnzhSRbTw.EBKKAJ3mxaU5I5_SIDEzXmba_TsxiB62vhadIQQqlIU"));
//     headers.insert("Sec-Ch-Ua", HeaderValue::from_static("\"Not?A_Brand\";v=\"8\", \"Chromium\";v=\"108\""));
//     headers.insert("X-Xbl-Contract-Version", HeaderValue::from_static("1"));
//     headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
//     headers.insert("Ms-Cv", HeaderValue::from_static("aoQ85DVCU6xVtkWKlFYaMG.0"));
//     headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("\"Linux\""));
//     headers.insert("Origin", HeaderValue::from_static("https://social.xbox.com"));
//     headers.insert("Sec-Fetch-Site", HeaderValue::from_static("cross-site"));
//     headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
//     headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
//     headers.insert("Referer", HeaderValue::from_static("https://social.xbox.com/"));
//     headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate"));
//     headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9"));
//     headers.insert("Connection", HeaderValue::from_static("close"));
//
//     // JSON Body
//     let body = r#"{
//         "gamertag":"yourmom1234",
//         "reservationId":"2535421559576896",
//         "targetGamertagFields":"gamertag"
//     }"#;
//
//     // // Proxy configuration
//     // let proxy = Proxy::http("http://127.0.0.1:8080")?;
//     let mut client = reqwest::Client::builder();
//     // client = client.proxy(proxy);
//     //
//     // // Load custom CA certificate (cert.der)
//     // let mut cert_file = File::open("src/cert.der")?;
//     // let mut cert_buf = Vec::new();
//     // cert_file.read_to_end(&mut cert_buf)?;
//     // let cert = reqwest::Certificate::from_der(&cert_buf)?;
//     // client = client.add_root_certificate(cert);
//
//     // Send the POST request
//     let client = client.build()?;
//     let res = client.post(url)
//         .headers(headers)
//         .body(body.to_string())
//         .send()
//         .await?;
//
//     // Output the result
//     // println!("{:#?}", res.text);
//     // Parse the response body as JSON
//     let response_body: GamertagReservationResponse = res.json().await?;
//
// // Access the properties in the response body
//     println!("Gamertag: {}", response_body.gamertag);
//     println!("Composed Gamertag: {}", response_body.composed_gamertag);
//     println!("Classic Gamertag: {}", response_body.classic_gamertag);
// // ... access other properties as needed
//
//
//     Ok(())
// }

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

