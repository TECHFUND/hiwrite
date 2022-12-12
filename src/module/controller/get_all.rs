use crate::module::model::Module;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::{pool_handler, Model};
use actix_web::{web, HttpResponse};

pub async fn get_modules(pool: web::Data<PGPool>) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;

    // Get modules
    let modules = Module::read_all(&postgres_pool)?;

    // Return modules
    Ok(HttpResponse::Created().json(modules))
}
