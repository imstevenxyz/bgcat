use surrealdb::{Surreal, engine::any::Any, Error};

use crate::db::models::Count;

pub async fn get_total_boardgame_count(db: &Surreal<Any>) -> Result<u32, Error> {
    let count: Option<Count> = db.query("SELECT count() FROM boardgame GROUP count").await?.take(0)?;
    Ok(count.unwrap_or(Count{count: 0}).count)
}

pub async fn get_total_expansion_count(db: &Surreal<Any>) -> Result<u32, Error>{
    let count: Vec<Count> = db.query("SELECT count(expansions) FROM boardgame").await?.take(0)?;
    let total: u32 = count.iter().map(|count| count.count).sum();
    Ok(total)
}