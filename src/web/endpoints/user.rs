use actix_files as fs;
use actix_web::web::{scope, Query, Path};
use actix_web::{get, web::Data, HttpResponse, Scope};
use tera::Context;

use crate::db::crud;
use crate::db::repo::SurrealDBRepo;
use crate::errors::BGCError;
use crate::prelude::WEBResult;
use crate::web::queries::BoardGameQuery;
use crate::{SETTINGS, TERA, TERA_C};

pub fn setup() -> Scope {
    let scope = scope("")
        .service(
            fs::Files::new("/static", "./static/")
                .use_etag(true)
                .use_last_modified(true),
        )
        .service(
            fs::Files::new("/assets", "./data/assets")
                .use_etag(true)
                .use_last_modified(true),
        )
        .service(index)
        .service(boardgame);
    scope
}

#[get("/")]
async fn index(mut params: Query<BoardGameQuery>, db: Data<SurrealDBRepo>) -> WEBResult {
    if params.limit == 0 {
        params.limit = SETTINGS.default_page_limit;
    }
    let boardgames = crud::boardgame_query(&db, &params.0).await?;
    let page_count =
        (crud::boardgame_count(&db, &params.0).await? as f32 / params.limit as f32).ceil() as u32;
    let query_string = &params
        .0
        .to_string()
        .replace(&format!("&page={}", params.page), "");

    let mut tera_ctx = Context::new();
    tera_ctx.clone_from(&TERA_C);
    tera_ctx.insert("search", &true);
    tera_ctx.insert("query", &params.into_inner());
    tera_ctx.insert("query_string", query_string);
    tera_ctx.insert("pagination_count", &page_count);
    tera_ctx.insert("ui_page_limit_step", &SETTINGS.ui_page_limit_step);
    tera_ctx.insert("boardgames", &boardgames);
    let render = TERA.render("pages/index.html", &tera_ctx).unwrap();
    Ok(HttpResponse::Ok().body(render))
}

#[get("/boardgame/{uid}")]
async fn boardgame(path: Path<String>, db: Data<SurrealDBRepo>) -> WEBResult {
    let uid = path.into_inner();
    let boardgame = crud::boardgame_read(&db, &uid).await?;


    match boardgame {
        None => Err(BGCError::NotFound("Boardgame not found".to_string())),
        Some(boardgame) => {
            let mut tera_ctx = Context::new();
            tera_ctx.clone_from(&TERA_C);
            tera_ctx.insert("boardgame", &boardgame);
            let render = TERA.render("pages/bg.html", &tera_ctx).unwrap();
            Ok(HttpResponse::Ok().body(render))
        }
    }
}
