use actix_web::http::header::Header;
use actix_web::web::{scope, Data, Json, Path, Query};
use actix_web::{delete, get, post, put, HttpRequest, HttpResponse, Scope};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};

use crate::db::{crud, stats};
use crate::db::models::BoardGame;
use crate::db::repo::SurrealDBRepo;
use crate::errors::BGCError;
use crate::prelude::{APIResult, GENResult};
use crate::web::queries::BoardGameQuery;
use crate::web::responses::StatisticsResponse;
use crate::{SETTINGS, utils};

pub fn setup() -> Scope {
    let scope = scope("/api/v1")
        .service(heartbeat)
        .service(query_boardgames)
        .service(create_boardgame)
        .service(read_boardgame)
        .service(update_boardgame)
        .service(delete_boardgame)
        .service(statistics);
    scope
}

fn is_authenticated(req: &HttpRequest) -> GENResult<()> {
    let is_auth = match Authorization::<Bearer>::parse(req) {
        Err(_) => false,
        Ok(auth) => auth.as_ref().token().eq(&SETTINGS.admin_token),
    };
    match is_auth {
        false => Err(BGCError::Unauthorized(
            "Invalid auth bearer header".to_string(),
        )),
        true => Ok(()),
    }
}

/// Check API status
#[utoipa::path(
    tag = "status",
    context_path = "/api/v1",
    responses(
        (status = 200, body = String)
    )
)]
#[get("/heartbeat")]
pub async fn heartbeat() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

/// Query boardgames
#[utoipa::path(
    tag = "public",
    context_path = "/api/v1",
    params(
        BoardGameQuery
    ),
    responses(
        (status = 200, body = Vec<BoardGame>)
    )
)]
#[get("/boardgames")]
async fn query_boardgames(params: Query<BoardGameQuery>, db: Data<SurrealDBRepo>) -> APIResult {
    let boardgames = crud::boardgame_query(&db, &params.0).await?;
    let page_count =
        (crud::boardgame_count(&db, &params.0).await? as f32 / params.limit as f32).ceil() as u32;

    if params.limit > 0 {
        Ok(HttpResponse::Ok()
            .insert_header(("Pagination-Count", page_count))
            .insert_header(("Pagination-Page", params.0.page))
            .insert_header(("Pagination-Limit", params.0.limit))
            .json(boardgames))
    } else {
        Ok(HttpResponse::Ok().json(boardgames))
    }
}

/// Create boardgame
#[utoipa::path(
    tag = "admin",
    context_path = "/api/v1",
    request_body = BoardGame,
    responses(
        (status = 200, body = BoardGame),
        (status = 401, body = ErrorMessageResponse)
    ),
    security(
        ("admin_token" = [])
    )
)]
#[post("/boardgames")]
async fn create_boardgame(
    req: HttpRequest,
    data: Json<BoardGame>,
    db: Data<SurrealDBRepo>,
) -> APIResult {
    is_authenticated(&req)?;

    let boardgame: BoardGame = crud::boardgame_create(&db, data.0).await?;
    Ok(HttpResponse::Created().json(boardgame))
}

/// Get boardgame by UID
#[utoipa::path(
    tag = "public",
    context_path = "/api/v1",
    responses(
        (status = 200, body = BoardGame),
        (status = 404, body = ErrorMessageResponse)
    )
)]
#[get("/boardgames/{uid}")]
async fn read_boardgame(path: Path<String>, db: Data<SurrealDBRepo>) -> APIResult {
    let uid = path.into_inner();
    let boardgame: Option<BoardGame> = crud::boardgame_read(&db, &uid).await?;
    if boardgame.is_none() {
        Err(BGCError::NotFound("Boardgame not found".to_string()))
    } else {
        Ok(HttpResponse::Ok().json(boardgame.unwrap()))
    }
}

/// Update boardgame
///
/// Warning: New data replaces the full boardgame!
#[utoipa::path(
    tag = "admin",
    context_path = "/api/v1",
    request_body = BoardGame,
    responses(
        (status = 200, body = BoardGame),
        (status = 401, body = ErrorMessageResponse),
        (status = 404, body = ErrorMessageResponse)
    ),
    security(
        ("admin_token" = [])
    )
)]
#[put("/boardgames/{uid}")]
async fn update_boardgame(
    req: HttpRequest,
    path: Path<String>,
    data: Json<BoardGame>,
    db: Data<SurrealDBRepo>,
) -> APIResult {
    is_authenticated(&req)?;

    let uid = path.into_inner();
    let mut data = data.0;
    data.uid = Some(uid);
    let boardgame: Option<BoardGame> = crud::boardgame_update(&db, data).await?;
    if boardgame.is_none() {
        Err(BGCError::NotFound("Boardgame not found".to_string()))
    } else {
        Ok(HttpResponse::Ok().json(boardgame))
    }
}

/// Delete boardgame
#[utoipa::path(
    tag = "admin",
    context_path = "/api/v1",
    responses(
        (status = 204),
        (status = 401, body = ErrorMessageResponse),
        (status = 404, body = ErrorMessageResponse)
    ),
    security(
        ("admin_token" = [])
    )
)]
#[delete("/boardgames/{uid}")]
async fn delete_boardgame(
    req: HttpRequest,
    path: Path<String>,
    db: Data<SurrealDBRepo>,
) -> APIResult {
    is_authenticated(&req)?;

    let uid = path.into_inner();
    let boardgame: Option<BoardGame> = crud::boardgame_delete(&db, &uid).await?;
    if boardgame.is_none() {
        Err(BGCError::NotFound("Boardgame not found".to_string()))
    } else {
        utils::delete_assets(&uid);
        Ok(HttpResponse::NoContent().finish())
    }
}

/// Get statistics
#[utoipa::path(
    tag = "public",
    context_path = "/api/v1",
    responses(
        (status = 200, body = StatisticsResponse),
    ),
)]
#[get("/statistics")]
async fn statistics(
    db: Data<SurrealDBRepo>,
) -> APIResult {
    let boardgame_count = stats::get_total_boardgame_count(&db.client).await?;
    let expansion_count = stats::get_total_expansion_count(&db.client).await?;
    let response = StatisticsResponse{
        boardgames: boardgame_count,
        expansions: expansion_count
    };
    Ok(HttpResponse::Ok().json(response))
}
