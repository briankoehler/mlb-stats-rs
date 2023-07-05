use serde::{Serialize, Deserialize};

use super::response::TeamsResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct TeamsRequest {
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
enum TeamsHydration {
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
        Self {
            season: Some("2023".into()), // TODO: Get current season
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
        // TODO: FIX THIS
        let params = serde_qs::to_string(self).unwrap();
        let params = "sportId=1&season=2023".to_string();
        reqwest::get(format!("https://statsapi.mlb.com/api/v1/teams?{}", params))
            .await.unwrap()
            .json::<TeamsResponse>()
            .await.unwrap()
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

    pub fn hydrate(&mut self, hydrate: Vec<TeamsHydration>) -> &mut Self {
        self.hydrate = Some(hydrate);
        self
    }
    
    pub fn fields(&mut self, fields: Vec<String>) -> &mut Self {
        self.fields = Some(fields);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_current_teams() {
        let request = TeamsRequest::default();
        request.send().await;
    }
}

