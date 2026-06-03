use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::pool_handler;
use crate::{module::model::ModuleCategory, utils::model_manager::Model};
use actix_web::{HttpResponse, web};

pub async fn get_category(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let mut postgres_pool = pool_handler(pool)?;

    // Get category
    let res = ModuleCategory::read_one(id.clone(), &mut postgres_pool)?;

    // Return category
    Ok(HttpResponse::Ok().json(res))
}
