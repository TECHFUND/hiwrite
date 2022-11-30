use crate::module::model::ModuleCategory;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use actix_web::{web, HttpResponse};

pub async fn get_module_category(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let modules = ModuleCategory::join(id.clone(), &postgres_pool)?;
    Ok(HttpResponse::Created().json(modules))
}
