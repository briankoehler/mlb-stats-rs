use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamsResponse {
    copyright: String,
    teams: Vec<TeamData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct TeamData {
    all_star_status: String,
    id: u32,
    name: String,
    link: String,
    season: u32, // TODO: Make a season type/struct for clear link
    // venue: TODO Venue response vs hydration
    team_code: String,
    file_code: String,
    abbreviation: String,
    team_name: String,
    location_name: String,
    first_year_of_play: String,
    // league: TODO
    // division: TODO
    // sport: TODO
    short_name: String,
    parent_org_name: Option<String>,
    parent_org_id: Option<u32>,
    franchise_name: String,
    club_name: String,
    active: bool,
}

