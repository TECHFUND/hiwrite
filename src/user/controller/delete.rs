use crate::utils::auth::Claims;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::pool_handler;
use crate::{user::model::User, utils::model_manager::Model};
use actix_web::{HttpResponse, web};

pub async fn delete_user(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let mut postgres_pool = pool_handler(pool)?;

    // Delete user
    let res = User::delete(id.clone(), &mut postgres_pool)?;

    // Return deleted user
    Ok(HttpResponse::Ok().json(res))
}
