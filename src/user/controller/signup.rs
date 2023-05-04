use std::ptr::null;

use super::login_res::login_res;
use crate::plugin::error;
use crate::user::model::MutUser;
use crate::user::model::User;
use crate::utils::auth::ErrorCodes;
use crate::utils::auth::encrypt_password;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordVerifier;
use uuid::Uuid;

pub async fn signup(
  user: web::Json<MutUser>,
  pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
  let postgres_pool = pool_handler(pool)?;

  if let Ok(_) = User::read_one(user.username.clone(), &postgres_pool) {
    Ok(HttpResponse::BadRequest().json("User already exists."))
  } else {
    let mut salted_user = user.clone();
    let encrypted_password = encrypt_password(&salted_user.password.unwrap())?;
    salted_user.password = Some(encrypted_password);
    salted_user.uuid = Some(Uuid::new_v4().to_string());

    User::create(&salted_user, &postgres_pool)?;

    Ok(HttpResponse::Created().json("You successfully signed up"))
  }
}
