use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameType {
    SpringTraining,
    RegularSeason,
    WildCardGame,
    DivisionSeries,
    LeagueChampionshipSeries,
    WorldSeries,
    Championship,
    NineteenthCenturySeries,
    Playoffs,
    #[serde(rename = "All-Star Game")]
    AllStarGame,
    Intrasquad,
    Exhibition,
}

