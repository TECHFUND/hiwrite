use crate::module::model::ModuleCategory;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::pool_handler;
use actix_web::{HttpResponse, web};

pub async fn get_category(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let mut postgres_pool = pool_handler(pool)?;

    // Get category
    let modules = ModuleCategory::join(id.clone(), &mut postgres_pool)?;

    // Return category
    Ok(HttpResponse::Created().json(modules))
}
