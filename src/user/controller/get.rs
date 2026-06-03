use crate::user::model::User;
use crate::utils::auth::Claims;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::pool_handler;
use actix_web::HttpResponse;
use actix_web::web;

pub async fn get_user(
    id: web::Path<String>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let mut postgres_pool = pool_handler(pool)?;

    // Get user
    let user: User = User::read_one(id.clone(), &mut postgres_pool)?;

    // Return user
    Ok(HttpResponse::Ok().json(&user))
}
