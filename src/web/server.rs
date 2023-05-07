use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::route;
use actix_web::{web::Data, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::db::repo::SurrealDBRepo;
use crate::errors::BGCError;
use crate::prelude::WEBResult;
use crate::web::endpoints::{admin, api_v1, user};
use crate::SETTINGS;

pub async fn server(db: &SurrealDBRepo) -> std::io::Result<()> {
    let db_data = Data::new(db.clone());
    let key = Key::derive_from(SETTINGS.secret.as_bytes());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                key.clone(),
            ))
            .app_data(db_data.clone())
            .service(
                SwaggerUi::new("/api/v1/docs/{_:.*}")
                    .url("/api/v1/openapi.json", api_v1::docs::ApiDoc::openapi()),
            )
            .service(api_v1::api::setup())
            .service(admin::setup())
            .service(user::setup())
            .default_service(route().to(default))
    })
    .bind((SETTINGS.adr.to_owned(), SETTINGS.port.clone()))?
    .run()
    .await
}

async fn default() -> WEBResult {
    Err(BGCError::NotFound("Default response".to_string()))
}
