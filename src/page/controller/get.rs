use crate::page::model::Page;
use crate::page::model::PageDTO;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::pool_handler;
use actix_web::HttpResponse;
use actix_web::web;

pub async fn get_page(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let mut postgres_pool = pool_handler(pool)?;

    // Get page
    let page: PageDTO = Page::read_one(id.clone(), &mut postgres_pool)?;

    // Return page
    Ok(HttpResponse::Ok().json(page))
}
