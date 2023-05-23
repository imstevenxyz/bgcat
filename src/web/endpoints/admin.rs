use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::web::{scope, Form, Path, Query};
use actix_web::{get, post, web::Data, HttpResponse, Scope};
use tera::Context;

use crate::db::crud;
use crate::db::models::BoardGame;
use crate::db::repo::SurrealDBRepo;
use crate::errors::BGCError;
use crate::prelude::{GENResult, WEBResult};
use crate::web::forms::{AdminAuthForm, BGExpansionForm, BGForm, BGMultiPartForm};
use crate::web::queries::BoardGameQuery;
use crate::{SETTINGS, TERA, TERA_C, utils};

pub fn setup() -> Scope {
    let scope = scope("/admin")
        .service(auth)
        .service(index)
        .service(bg_create_form)
        .service(bg_create)
        .service(bg_update_form)
        .service(bg_update)
        .service(bg_delete);
    scope
}

fn is_authenticated(session: &Session) -> GENResult<bool> {
    let result: bool = session.get::<bool>("authenticated")?.unwrap_or(false);
    if result || SETTINGS.debug {
        Ok(true)
    } else {
        Err(BGCError::Unauthorized("Invalid session".to_string()))
    }
}

#[get("")]
async fn index(
    session: Session,
    params: Query<BoardGameQuery>,
    db: Data<SurrealDBRepo>,
) -> WEBResult {
    let is_authenticated: bool = session.get::<bool>("authenticated")?.unwrap_or(false);
    if is_authenticated {
        Ok(admin_index(params, db).await?)
    } else {
        Ok(admin_auth_form()?)
    }
}

fn admin_auth_form() -> WEBResult {
    let mut tera_ctx = Context::new();
    tera_ctx.clone_from(&TERA_C);
    let render = TERA.render("pages/auth.html", &tera_ctx).unwrap();
    Ok(HttpResponse::Ok().body(render))
}

async fn admin_index(mut params: Query<BoardGameQuery>, db: Data<SurrealDBRepo>) -> WEBResult {
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
    tera_ctx.insert("boardgames", &boardgames);
    let render = TERA.render("pages/admin.html", &tera_ctx).unwrap();
    Ok(HttpResponse::Ok().body(render))
}

#[post("")]
async fn auth(session: Session, Form(form): Form<AdminAuthForm>) -> WEBResult {
    if form.token.eq(&SETTINGS.admin_token) || SETTINGS.debug {
        session.insert("authenticated", true).unwrap(); //TODO FIX ERROR RETURN
        Ok(HttpResponse::Found()
            .append_header(("Location", "/admin"))
            .finish())
    } else {
        Err(BGCError::Unauthorized("Invalid token".to_string()))
    }
}

#[get("/new")]
async fn bg_create_form(session: Session) -> WEBResult {
    is_authenticated(&session)?;

    let form = BGForm {
        title: None,
        image_url: None,
        min_players: None,
        max_players: None,
        players_no_limit: None,
        min_playtime: None,
        max_playtime: None,
        playtime_no_limit: None,
        expansions: Vec::new(),
    };

    let mut tera_ctx = Context::new();
    tera_ctx.clone_from(&TERA_C);
    tera_ctx.insert("form", &form);
    tera_ctx.insert("formtype", "create");
    let render = TERA.render("pages/bg_form.html", &tera_ctx).unwrap();
    Ok(HttpResponse::Ok().body(render))
}

#[post("/new")]
async fn bg_create(
    session: Session,
    form: MultipartForm<BGMultiPartForm>,
    db: Data<SurrealDBRepo>,
) -> WEBResult {
    is_authenticated(&session)?;

    let form: BGMultiPartForm = form.into_inner();
    match form.meta_add_expansion {
        Some(_) => Ok(bg_create_add_expansion_form(form).await?),
        None => Ok(bg_create_submit(form, db).await?),
    }
}

async fn bg_create_add_expansion_form(form: BGMultiPartForm) -> WEBResult {
    let mut form = BGForm::from(form);
    form.expansions.push(BGExpansionForm {
        title: None,
    });

    let mut tera_ctx = Context::new();
    tera_ctx.clone_from(&TERA_C);
    tera_ctx.insert("form", &form);
    tera_ctx.insert("formtype", "create");
    let render = TERA.render("pages/bg_form.html", &tera_ctx).unwrap();
    Ok(HttpResponse::Ok().body(render))
}

async fn bg_create_submit(form: BGMultiPartForm, db: Data<SurrealDBRepo>) -> WEBResult {
    let uid = crud::generate_boardgame_uid(&form.title);
    let image_url = form.save_image(&uid)?;
    let mut new_bg = BoardGame::from(form);
    new_bg.uid = Some(uid);
    if image_url.is_some() {
        new_bg.image_url = image_url.unwrap();
    }

    let _bg = crud::boardgame_create(&db, new_bg).await?; //TODO show info page
    Ok(HttpResponse::Found()
        .append_header(("Location", "/admin"))
        .finish())
}

#[get("/edit/{uid}")]
async fn bg_update_form(
    session: Session,
    path: Path<String>,
    db: Data<SurrealDBRepo>,
) -> WEBResult {
    is_authenticated(&session)?;

    let uid = path.into_inner();
    let boardgame = crud::boardgame_read(&db, &uid).await?;

    match boardgame {
        None => Err(BGCError::NotFound("Boardgame not found".to_string())),
        Some(boardgame) => {
            let form = BGForm::from(boardgame);

            let mut tera_ctx = Context::new();
            tera_ctx.clone_from(&TERA_C);
            tera_ctx.insert("form", &form);
            tera_ctx.insert("formtype", "update");
            let render = TERA.render("pages/bg_form.html", &tera_ctx).unwrap();
            Ok(HttpResponse::Ok().body(render))
        }
    }
}

#[post("/edit/{uid}")]
async fn bg_update(
    session: Session,
    path: Path<String>,
    form: MultipartForm<BGMultiPartForm>,
    db: Data<SurrealDBRepo>,
) -> WEBResult {
    is_authenticated(&session)?;

    let uid = path.into_inner();
    let boardgame = crud::boardgame_read(&db, &uid).await?;

    match boardgame {
        None => Err(BGCError::NotFound("Boardgame not found".to_string())),
        Some(boardgame) => {
            let form: BGMultiPartForm = form.into_inner();
            match form.meta_add_expansion {
                Some(_) => Ok(bg_update_add_expansion_form(boardgame, form).await?),
                None => Ok(bg_update_submit(boardgame, form, db).await?),
            }
        }
    }
}

async fn bg_update_add_expansion_form(bg: BoardGame, form: BGMultiPartForm) -> WEBResult {
    let mut form = BGForm::from(form);
    form.image_url = Some(bg.image_url);
    form.expansions.push(BGExpansionForm {
        title: None,
    });

    let mut tera_ctx = Context::new();
    tera_ctx.clone_from(&TERA_C);
    tera_ctx.insert("form", &form);
    tera_ctx.insert("formtype", "update");
    let render = TERA.render("pages/bg_form.html", &tera_ctx).unwrap();
    Ok(HttpResponse::Ok().body(render))
}

async fn bg_update_submit(
    bg: BoardGame,
    mut form: BGMultiPartForm,
    db: Data<SurrealDBRepo>,
) -> WEBResult {
    let image_url = form.save_image(bg.uid.as_ref().unwrap())?;

    let mut indices: Vec<u8> = Vec::new();
    form.meta_del_expansion.reverse();
    for val in form.meta_del_expansion.iter() {
        indices.push(val.0);
    }
    for i in indices {
        form.expansion_titles.remove(i.into());
    }

    let mut new_bg = BoardGame::from(form);
    new_bg.uid = bg.uid;
    if image_url.is_some() {
        new_bg.image_url = image_url.unwrap();
    } else {
        new_bg.image_url = bg.image_url;
    }

    let _bg = crud::boardgame_update(&db, new_bg).await?; //TODO show info page
    Ok(HttpResponse::Found()
        .append_header(("Location", "/admin"))
        .finish())
}

#[post("/delete/{uid}")]
async fn bg_delete(session: Session, path: Path<String>, db: Data<SurrealDBRepo>) -> WEBResult {
    is_authenticated(&session)?;

    let uid = path.into_inner();
    let boardgame: Option<BoardGame> = crud::boardgame_delete(&db, &uid).await?;

    if boardgame.is_none() {
        Err(BGCError::NotFound("Boardgame not found".to_string()))
    } else {
        utils::delete_assets(&uid);
        Ok(HttpResponse::Found()
        .append_header(("Location", "/admin"))
        .finish())
    }
}
