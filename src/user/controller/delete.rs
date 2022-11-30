use crate::utils::auth::Claims;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use crate::{user::model::User, utils::model_manager::Model};
use actix_web::{web, HttpResponse};

pub async fn delete_user(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let res = User::delete(id.clone(), &postgres_pool)?;
    Ok(HttpResponse::Ok().json(res))
}
