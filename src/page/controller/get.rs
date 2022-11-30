use crate::page::model::Page;
use crate::page::model::PageDTO;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn get_page(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;

    let page: PageDTO = Page::read_one(id.clone(), &postgres_pool)?;
    Ok(HttpResponse::Ok().json(page))
}
