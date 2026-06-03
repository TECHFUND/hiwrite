use crate::utils::auth::Claims;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::pool_handler;
use crate::{module::model::Module, utils::model_manager::Model};
use actix_web::{HttpResponse, web};

pub async fn delete_module(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let mut postgres_pool = pool_handler(pool)?;

    // Delete module
    let res = Module::delete(id.clone(), &mut postgres_pool)?;

    // Return deleted module
    Ok(HttpResponse::Created().json(res))
}
