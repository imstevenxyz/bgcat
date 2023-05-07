use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::db::models::{BoardGame, BoardGameExpansion, DbQueryField, DbQuerySortDirection};
use crate::web::endpoints::api_v1::api;
use crate::web::responses::ErrorMessageResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::heartbeat,
        api::query_boardgames,
        api::create_boardgame,
        api::read_boardgame,
        api::update_boardgame,
        api::delete_boardgame
    ),
    components(
        schemas(
            BoardGame,
            BoardGameExpansion,
            DbQueryField,
            DbQuerySortDirection,
            ErrorMessageResponse,
        )
    ),
    modifiers(&ApiAuthDoc)
)]
pub struct ApiDoc;

pub struct ApiAuthDoc;
impl Modify for ApiAuthDoc {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "admin_token",
            SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
        )
    }
}
