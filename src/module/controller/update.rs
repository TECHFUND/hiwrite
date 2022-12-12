use crate::module::model::MutModule;
use crate::utils::auth::Claims;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use crate::{module::model::Module, utils::model_manager::Model};
use actix_web::{web, HttpResponse};

pub async fn update_module(
    updated_module: web::Json<MutModule>,
    id: web::Path<String>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;

    // Update module
    Module::update(id.clone(), &updated_module, &postgres_pool)?;

    // Return updated module
    Ok(HttpResponse::Created().json(updated_module.0))
}
