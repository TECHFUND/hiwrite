use crate::page::model::Page;
use crate::page::model::PageDTO;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn get_pages(pool: web::Data<PGPool>) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;

    // Get pages
    let pages: Vec<PageDTO> = Page::read_all(&postgres_pool)?;

    // Return pages
    Ok(HttpResponse::Ok().json(pages))
}
