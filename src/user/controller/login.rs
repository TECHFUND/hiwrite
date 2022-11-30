use super::login_res::login_res;
use crate::user::model::MutUser;
use crate::user::model::User;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordVerifier;

pub async fn login(
    user: web::Json<MutUser>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let arg = Argon2::default();
    let read_user = User::read_one(user.username.clone(), &postgres_pool)?;
    let is_default = read_user.username == "root" && read_user.password == "";
    if read_user.token.is_some() && is_default {
        return Ok(HttpResponse::Forbidden().finish());
    }
    if is_default {
        let mut new_user = user.clone();
        let cookie = login_res(&mut new_user)?;
        let cookie_response = HttpResponse::Accepted().cookie(cookie.clone()).finish();
        new_user.token = Some(cookie.value().to_string());
        User::update_with_token(&new_user, &postgres_pool)?;
        return Ok(cookie_response);
    }
    let read_user_password = PasswordHash::new(&read_user.password).unwrap();

    match arg.verify_password(
        user.password.clone().unwrap().as_bytes(),
        &read_user_password,
    ) {
        Ok(_) => {
            let mut new_user = user;
            let cookie = login_res(&mut new_user)?;
            let cookie_response = HttpResponse::Ok().cookie(cookie.clone()).finish();
            new_user.token = Some(cookie.value().to_string());
            User::update_with_token(&new_user, &postgres_pool)?;
            Ok(cookie_response)
        }
        _ => Ok(HttpResponse::Unauthorized().json("Failed to authenticate.")),
    }
}
