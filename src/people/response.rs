use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PeopleResponse {
    copyright: String,
    people: Vec<PeopleData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PeopleData {
    id: u32,
    full_name: String,
    link: String,
    first_name: String,
    last_name: String,
    primary_number: String,
    birth_date: String,
    current_age: u32,
    birth_city: String,
    height: String,
    weight: u32,
    active: bool,
    // TODO: primary_position: _
    use_name: String,
    use_last_name: String,
    middle_name: String,
    boxscore_name: String,
    nick_name: String,
    gender: String,
    is_player: bool,
    is_verified: bool,
    draft_year: u32,
    mlb_debut_date: String,
    // TODO: bat_side: _
    // TODO: pitch_hand: _
    name_first_last: String,
    name_slug: String,
    first_last_name: String,
    last_first_name: String,
    last_init_name: String,
    #[serde(rename = "fullFMLName")]
    full_fml_name: String,
    #[serde(rename = "fullLFMName")]
    full_lfm_name: String,
    strike_zone_top: f64,
    strike_zone_bottom: f64,
}
