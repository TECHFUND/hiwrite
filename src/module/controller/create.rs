use crate::module::model::Module;
use crate::module::model::MutModule;
use crate::utils::auth::Claims;
use crate::utils::error::CustomHttpError;
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
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let mut uuid_new = new.clone();
    uuid_new.uuid = Some(Uuid::new_v4().to_string());
    Module::create(&uuid_new, &postgres_pool)?;
    Ok(HttpResponse::Created().json(uuid_new))
}
