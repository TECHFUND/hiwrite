use crate::user::model::User;
use crate::utils::auth::authenticate;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

pub async fn check_login(
    req: HttpRequest,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;

    // Check the authorization header
    let auth_header = req.headers().get("authorization");
    let auth_res = authenticate(auth_header.unwrap(), &postgres_pool).await;

    // Return the result
    match auth_res {
        Ok(u) => {
            let user = User::read_one(u.sub.clone(), &postgres_pool)?;
            Ok(HttpResponse::Ok().json(user))
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish()),
    }
}
