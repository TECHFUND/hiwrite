use crate::module::model::ModuleCategory;
use crate::module::model::MutCategory;
use crate::utils::auth::Claims;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn update_category(
    updated_category: web::Json<MutCategory>,
    id: web::Path<String>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    ModuleCategory::update(id.clone(), &updated_category, &postgres_pool)?;
    Ok(HttpResponse::Ok().json(updated_category.0))
}
