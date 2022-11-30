use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use crate::{module::model::ModuleCategory, utils::model_manager::Model};
use actix_web::{web, HttpResponse};

pub async fn get_category(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let res = ModuleCategory::read_one(id.clone(), &postgres_pool)?;
    Ok(HttpResponse::Ok().json(res))
}
