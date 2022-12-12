use crate::page::model::Page;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn delete_page(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;
    // Delete page
    let res = Page::delete(id.clone(), &postgres_pool)?;

    // Return deleted page
    Ok(HttpResponse::Ok().json(res))
}
