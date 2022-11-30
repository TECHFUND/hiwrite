use crate::utils::auth::Claims;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use crate::{module::model::Module, utils::model_manager::Model};
use actix_web::{web, HttpResponse};

pub async fn delete_module(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let res = Module::delete(id.clone(), &postgres_pool)?;
    Ok(HttpResponse::Created().json(res))
}
