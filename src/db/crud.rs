use slug::slugify;
use surrealdb::Error;

use crate::db::models::{BoardGame, Count};
use crate::db::query::DbQuery;
use crate::db::repo::SurrealDBRepo;
use crate::web::queries::BoardGameQuery;

pub async fn boardgame_create(db: &SurrealDBRepo, mut data: BoardGame) -> Result<BoardGame, Error> {
    data.uid = Some(slugify(&data.title));
    let boardgame: BoardGame = db
        .client
        .create(("boardgame", data.uid.clone().unwrap()))
        .content(data)
        .await?;
    Ok(boardgame)
}

pub async fn boardgame_read(db: &SurrealDBRepo, uid: &str) -> Result<Option<BoardGame>, Error> {
    let boardgame: Option<BoardGame> = db.client.select(("boardgame", uid)).await?;
    Ok(boardgame)
}

pub async fn boardgame_update(
    db: &SurrealDBRepo,
    data: BoardGame,
) -> Result<Option<BoardGame>, Error> {
    let boardgame: Option<BoardGame> = db
        .client
        .update(("boardgame", data.uid.clone().unwrap()))
        .content(data)
        .await?;
    Ok(boardgame)
}

pub async fn boardgame_delete(db: &SurrealDBRepo, uid: &str) -> Result<Option<BoardGame>, Error> {
    let boardgame: Option<BoardGame> = db.client.delete(("boardgame", uid)).await?;
    Ok(boardgame)
}

pub async fn boardgame_read_all(db: &SurrealDBRepo) -> Result<Vec<BoardGame>, Error> {
    let boardgames: Vec<BoardGame> = db.client.select("boardgame").await?;
    Ok(boardgames)
}

pub async fn boardgame_count(db: &SurrealDBRepo, query: &BoardGameQuery) -> Result<u32, Error> {
    let query = DbQuery::count(query);
    let count: Option<Count> = query.execute(db.client).await?.take(0)?;
    Ok(count.unwrap_or(Count { count: 0 }).count)
}

pub async fn boardgame_query(
    db: &SurrealDBRepo,
    query: &BoardGameQuery,
) -> Result<Vec<BoardGame>, Error> {
    let query = DbQuery::select(query);
    let boardgames: Vec<BoardGame> = query.execute(db.client).await?.take(0)?;
    Ok(boardgames)
}
