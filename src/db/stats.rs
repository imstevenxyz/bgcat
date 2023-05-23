use surrealdb::Error;

use crate::db::models::Count;
use crate::db::repo::SurrealDBRepo;
use crate::web::responses::StatisticsResponse;

pub async fn get_all_statistics(db: &SurrealDBRepo) -> Result<StatisticsResponse, Error> {
    let boardgames = get_total_boardgame_count(db).await?;
    let expansions = get_total_expansion_count(db).await?;
    Ok(StatisticsResponse {
        boardgames,
        expansions,
    })
}

pub async fn get_total_boardgame_count(db: &SurrealDBRepo) -> Result<u32, Error> {
    let count: Option<Count> = db
        .client
        .query("SELECT count() FROM boardgame GROUP count")
        .await?
        .take(0)?;
    Ok(count.unwrap_or(Count { count: 0 }).count)
}

pub async fn get_total_expansion_count(db: &SurrealDBRepo) -> Result<u32, Error> {
    let count: Vec<Count> = db
        .client
        .query("SELECT count(expansions) FROM boardgame")
        .await?
        .take(0)?;
    let total: u32 = count.iter().map(|count| count.count).sum();
    Ok(total)
}
