use crate::page::model::Page;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::pool_handler;
use actix_web::HttpResponse;
use actix_web::web;

pub async fn get_page_join_modules(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let mut postgres_pool = pool_handler(pool)?;

    // Get page
    let page_vec = Page::read_one_join_on(id.clone(), &mut postgres_pool)?;

    // Return page
    Ok(HttpResponse::Ok().json(page_vec))
}
