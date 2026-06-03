use crate::module::model::Module;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::{Model, pool_handler};
use actix_web::{HttpResponse, web};

pub async fn get_modules(pool: web::Data<PGPool>) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let mut postgres_pool = pool_handler(pool)?;

    // Get modules
    let modules = Module::read_all(&mut postgres_pool)?;

    // Return modules
    Ok(HttpResponse::Created().json(modules))
}
