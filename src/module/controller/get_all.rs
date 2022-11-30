use crate::module::model::Module;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::{pool_handler, Model};
use actix_web::{web, HttpResponse};

pub async fn get_modules(pool: web::Data<PGPool>) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let modules = Module::read_all(&postgres_pool)?;
    Ok(HttpResponse::Created().json(modules))
}
