use super::response::TeamsResponse;
use chrono::Datelike;
use serde::{Deserialize, Serialize};

pub const MLB_SPORT_ID: u32 = 1;
pub const AL_LEAGUE_ID: u32 = 103;
pub const NL_LEAGUE_ID: u32 = 104;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamsRequest {
    /// Typically the year
    season: Option<String>,
    // activeStatus: Option<>,
    league_ids: Option<Vec<u32>>,
    sport_ids: Option<Vec<u32>>,
    // game_type: Option<GameType>,
    hydrate: Option<Vec<TeamsHydration>>,
    /// Comma-separated tokens of fields to accept
    /// TODO: Define this further
    fields: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TeamsHydration {
    PreviousSchedule,
    NextSchedule,
    Venue,
    SpringVenue,
    Social,
    DeviceProperties,
    #[serde(rename = "game(promotions)")]
    GamePromotions,
    #[serde(rename = "game(atBatPromotions)")]
    GameAtBatPromotions,
    #[serde(rename = "game(tickets)")]
    GameTickets,
    #[serde(rename = "game(atBatTickets)")]
    GameAtBatTickets,
    #[serde(rename = "game(sponsorships)")]
    GameSponsorships,
    League,
    Person,
    Sport,
    Standings,
    Division,
    XrefId,
    Location,
}

impl Default for TeamsRequest {
    fn default() -> Self {
        let current_year = chrono::Utc::now().year();
        Self {
            season: Some(current_year.to_string()),
            league_ids: None,
            sport_ids: Some(vec![1]),
            hydrate: None,
            fields: None,
        }
    }
}

impl TeamsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn send(&self) -> TeamsResponse {
        let params = self.to_query_string();
        reqwest::get(format!("https://statsapi.mlb.com/api/v1/teams?{}", params))
            .await
            .unwrap()
            .json::<TeamsResponse>()
            .await
            .unwrap()
    }

    pub fn season(&mut self, id: String) -> &mut Self {
        self.season = Some(id);
        self
    }

    pub fn league_ids(&mut self, league_ids: Vec<u32>) -> &mut Self {
        self.league_ids = Some(league_ids);
        self
    }

    pub fn sport_ids(&mut self, sport_ids: Vec<u32>) -> &mut Self {
        self.sport_ids = Some(sport_ids);
        self
    }

    #[deprecated(note = "Hydrations not yet supported.")]
    pub fn hydrate(&mut self, hydrate: Vec<TeamsHydration>) -> &mut Self {
        self.hydrate = Some(hydrate);
        self
    }

    #[deprecated(note = "Fields not yet supported.")]
    pub fn fields(&mut self, fields: Vec<String>) -> &mut Self {
        self.fields = Some(fields);
        self
    }

    fn to_query_string(&self) -> String {
        let mut params = vec![];
        if let Some(season) = self.season.clone() {
            params.push(format!("season={}", season).to_owned());
        }
        if let Some(league_ids) = self.league_ids.clone() {
            params.push(format!("leagueIds={}", ids_to_string(league_ids)).to_owned());
        }
        if let Some(sport_ids) = self.sport_ids.clone() {
            params.push(format!("sportIds={}", ids_to_string(sport_ids)).to_owned());
        }

        params.join("&")
    }
}

fn ids_to_string(ids: Vec<u32>) -> String {
    ids.into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test only works if able to reach the API
    // #[tokio::test]
    // async fn get_current_teams() {
    //     let request = TeamsRequest::default();
    //     request.send().await;
    // }

    #[test]
    fn valid_query_string() {
        let query_string = TeamsRequest::new()
            .sport_ids(vec![MLB_SPORT_ID])
            .league_ids(vec![AL_LEAGUE_ID, NL_LEAGUE_ID])
            .to_query_string();

        let params: Vec<_> = query_string.split("&").collect();
        println!("{:?}", params);
        assert!(params.contains(&"season=2023"));
        assert!(params.contains(&"leagueIds=103,104"));
        assert!(params.contains(&"sportIds=1"));
        assert_eq!(params.len(), 3);
    }
}
