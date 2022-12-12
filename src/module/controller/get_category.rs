use crate::module::model::ModuleCategory;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use actix_web::{web, HttpResponse};

pub async fn get_module_category(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;

    // Get category
    let modules = ModuleCategory::join(id.clone(), &postgres_pool)?;

    // Return category
    Ok(HttpResponse::Created().json(modules))
}
