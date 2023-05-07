use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::SETTINGS;
use crate::prelude::GENResult;

pub static DB: Surreal<Any> = Surreal::init();

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub client: &'static Surreal<Any>,
}

impl SurrealDBRepo {
    pub async fn new() -> GENResult<Self> {
        DB.connect(&SETTINGS.db_adr).await?;
        DB.signin(Root {
            username: &SETTINGS.db_user,
            password: &SETTINGS.db_pass,
        }).await?;
        DB.use_ns(&SETTINGS.db_ns).use_db(&SETTINGS.db_name).await?;
        Ok(SurrealDBRepo { client: &DB })
    }
}
