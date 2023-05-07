use core::fmt;
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::IntoParams;

use crate::db::models::{DbQueryField, DbQuerySortDirection};

#[derive(Serialize, Deserialize, IntoParams, Debug)]
pub struct BoardGameQuery {
    #[serde(default)]
    pub limit: u32,
    #[serde(default)]
    pub page: u32,
    #[serde(default)]
    pub sort_by: DbQueryField,
    #[serde(default)]
    pub sort_direction: DbQuerySortDirection,
    pub search: Option<String>,
    pub genre: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub players: Option<u32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub min_playtime: Option<u32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub max_playtime: Option<u32>,
}

pub fn empty_string_as_none<'de, T, D>(d: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(T::deserialize(d).ok())
}

impl fmt::Display for BoardGameQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut query = serde_urlencoded::to_string(&self).unwrap_or("".to_string());
        query = query
            .replace("&sort_direction=asc", "")
            .replace("&sort_by=title", "")
            .replace("&page=0", "")
            .replace("limit=0", "");

        if query.starts_with("&") {
            query.remove(0);
        }

        write!(f, "{}", query)
    }
}
