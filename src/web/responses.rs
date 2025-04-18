use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ErrorMessageResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct StatisticsResponse {
    pub boardgames: u32,
    pub expansions: u32,
    pub boardgames_available: u32,
    pub boardgames_unavailable: u32,
}
