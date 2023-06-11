use super::login_res::login_res;
use crate::user::model::MutUser;
use crate::user::model::User;
use crate::utils::error::HttpErrorCodes;
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
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;
    let arg = Argon2::default();

    // Get user
    let read_user = User::read_one(user.username.clone(), &postgres_pool)?;
    // I set the password "root" when signup
    let is_default = read_user.username == "root" && read_user.password == "root";
    if read_user.token.is_some() && is_default {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // if user is default, create a new token
    if is_default {
        let mut new_user = user.clone();
        let cookie = login_res(&mut new_user)?;
        let cookie_response = HttpResponse::Accepted().cookie(cookie.clone()).finish();
        new_user.token = Some(cookie.value().to_string());
        User::update_with_token(&new_user, &postgres_pool)?;
        let read_user = User::read_one(new_user.username.clone(), &postgres_pool)?;
        return Ok(HttpResponse::Ok().json(read_user));
    }

    // Verify password
    let read_user_password = PasswordHash::new(&read_user.password).unwrap();

    // Match password and return token
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
            let read_user = User::read_one(new_user.username.clone(), &postgres_pool)?;
            Ok(HttpResponse::Ok().json(read_user))
        }
        _ => Ok(HttpResponse::Unauthorized().json("Failed to authenticate.")),
    }
}
