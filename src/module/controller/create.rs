use crate::module::model::Module;
use crate::module::model::MutModule;
use crate::utils::auth::Claims;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;
use uuid::Uuid;

pub async fn create_module(
    new: web::Json<MutModule>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;

    // Create new module
    let mut uuid_new = new.clone();
    uuid_new.uuid = Some(Uuid::new_v4().to_string());
    Module::create(&uuid_new, &postgres_pool)?;

    // Return new module
    Ok(HttpResponse::Created().json(uuid_new))
}
