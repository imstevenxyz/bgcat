use core::fmt;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct BoardGame {
    pub uid: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub image_url: String,
    #[serde(default)]
    pub min_players: u8,
    #[serde(default)]
    pub max_players: u8,
    #[serde(default)]
    #[schema(default = false)]
    pub players_no_limit: bool,
    #[serde(default)]
    pub min_playtime: u16,
    #[serde(default)]
    pub max_playtime: u16,
    #[serde(default)]
    #[schema(default = false)]
    pub playtime_no_limit: bool,
    #[serde(default)]
    #[schema(default = true)]
    pub available: bool,
    #[serde(default)]
    pub expansions: Vec<BoardGameExpansion>,
    pub bgg_id: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct BoardGameExpansion {
    pub title: String,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DbQuerySortDirection {
    #[default]
    ASC,
    DESC,
}

impl fmt::Display for DbQuerySortDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbQuerySortDirection::ASC => write!(f, "asc"),
            DbQuerySortDirection::DESC => write!(f, "desc"),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DbQueryField {
    #[default]
    Title,
    MinPlayers,
    MaxPlayers,
    MinPlaytime,
    MaxPlaytime,
    Genre,
    Available,
}

impl fmt::Display for DbQueryField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbQueryField::Title => write!(f, "title"),
            DbQueryField::MinPlayers => write!(f, "min_players"),
            DbQueryField::MaxPlayers => write!(f, "max_players"),
            DbQueryField::MinPlaytime => write!(f, "min_playtime"),
            DbQueryField::MaxPlaytime => write!(f, "max_playtime"),
            DbQueryField::Genre => write!(f, "genre"),
            DbQueryField::Available => write!(f, "available"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Count {
    pub count: u32,
}
