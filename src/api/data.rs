use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GamertagReservationResponse {
    pub prompt_for_classic_gamertag: bool,
    pub classic_translation_level: String,
    pub gamertag_suffix: String,
    pub gamertag: String,
    pub composed_gamertag: String,
    pub classic_gamertag: String,
}
