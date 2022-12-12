use crate::page::model::MutPage;
use crate::page::model::Page;
use crate::utils::auth::Claims;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn update_page(
    updated_page: web::Json<MutPage>,
    id: web::Path<String>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;

    // Update page
    Page::update(id.clone(), &updated_page, &postgres_pool)?;

    // Return updated page
    Ok(HttpResponse::Ok().json(updated_page.0))
}
