use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use crate::{module::model::Module, utils::model_manager::Model};
use actix_web::{web, HttpResponse};

pub async fn get_module(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;

    // Get module
    let module = Module::read_one(id.clone(), &postgres_pool)?;

    // Return module
    Ok(HttpResponse::Created().json(module))
}
