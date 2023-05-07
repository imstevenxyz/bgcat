pub mod db;
pub mod errors;
pub mod prelude;
pub mod settings;
pub mod utils;
pub mod web;

use lazy_static::lazy_static;
use log::{debug, info};
use prelude::GENResult;

use crate::db::proc::init_local_db;
use crate::db::repo::SurrealDBRepo;
use crate::settings::Settings;
use crate::web::server;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::load();
    pub static ref TERA: tera::Tera = setup_template_engine();
    pub static ref TERA_C: tera::Context = setup_template_context();
}

#[actix_web::main]
async fn main() -> GENResult<()> {
    Settings::setup_logger(&SETTINGS.log_level);
    debug!("Configuration: {:?}", *SETTINGS);
    info!("Running BGCAT: {}", settings::VERSION);
    utils::setup_data_dir()?;
    let db_proc = init_local_db()?;
    let db_repo = SurrealDBRepo::new().await?;

    server::server(&db_repo).await?;
    drop(db_proc);
    Ok(())
}

fn setup_template_engine() -> tera::Tera {
    match tera::Tera::new("templates/**/*") {
        Ok(tera) => tera,
        Err(why) => panic!("{}", why),
    }
}

fn setup_template_context() -> tera::Context {
    let mut context = tera::Context::new();
    context.insert("version", settings::VERSION);
    context.insert("css_ver", settings::VERSION);
    context.insert("js_ver", settings::VERSION);
    context
}
