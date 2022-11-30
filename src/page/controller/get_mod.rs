use crate::page::model::Page;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn get_page_join_modules(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let page_vec = Page::read_one_join_on(id.clone(), &postgres_pool)?;
    Ok(HttpResponse::Ok().json(page_vec))
}
