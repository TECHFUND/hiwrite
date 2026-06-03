use crate::page::model::Page;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::pool_handler;
use actix_web::HttpResponse;
use actix_web::web;

pub async fn delete_page(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let mut postgres_pool = pool_handler(pool)?;
    // Delete page
    let res = Page::delete(id.clone(), &mut postgres_pool)?;

    // Return deleted page
    Ok(HttpResponse::Ok().json(res))
}
