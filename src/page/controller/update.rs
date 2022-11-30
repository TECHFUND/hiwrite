use crate::page::model::MutPage;
use crate::page::model::Page;
use crate::utils::auth::Claims;
use crate::utils::error::CustomHttpError;
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
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;

    Page::update(id.clone(), &updated_page, &postgres_pool)?;

    Ok(HttpResponse::Ok().json(updated_page.0))
}
