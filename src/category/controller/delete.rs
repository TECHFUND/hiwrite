use crate::module::model::ModuleCategory;
use crate::utils::auth::Claims;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn delete_category(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let res = ModuleCategory::delete(id.clone(), &postgres_pool)?;
    Ok(HttpResponse::Ok().json(res))
}
