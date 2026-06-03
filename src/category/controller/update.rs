use crate::module::model::ModuleCategory;
use crate::module::model::MutCategory;
use crate::utils::auth::Claims;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::pool_handler;
use actix_web::HttpResponse;
use actix_web::web;

pub async fn update_category(
    updated_category: web::Json<MutCategory>,
    id: web::Path<String>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let mut postgres_pool = pool_handler(pool)?;

    // Update category
    ModuleCategory::update(id.clone(), &updated_category, &mut postgres_pool)?;

    // Return updated category
    Ok(HttpResponse::Ok().json(updated_category.0))
}
