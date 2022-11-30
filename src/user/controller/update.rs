use crate::user::model::MutUser;
use crate::user::model::User;
use crate::utils::auth::encrypt;
use crate::utils::auth::encrypt_password;
use crate::utils::auth::Claims;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::cookie::Cookie;
use actix_web::web;
use actix_web::HttpResponse;
use time::Duration;
use time::OffsetDateTime;

pub async fn update_user(
    id: web::Path<String>,
    new: web::Json<MutUser>,
    pool: web::Data<PGPool>,
    claim: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let mut salted_user = new.clone();
    if id.clone() != claim.sub {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let encrypted_password = encrypt_password(&salted_user.password.unwrap())?;
    salted_user.password = Some(encrypted_password);
    let exp_time = chrono::Utc::now() + chrono::Duration::days(10);
    let claim = Claims {
        exp: (exp_time).timestamp() as usize,
        sub: salted_user.username.clone(),
    };
    let time: OffsetDateTime = OffsetDateTime::now_utc() + Duration::hour();
    let token_enc = encrypt(claim)?;
    let cookie = Cookie::build("auth", &token_enc)
        .expires(time)
        .path("/")
        .finish();
    let user = HttpResponse::Ok().cookie(cookie).json(&new.clone());
    salted_user.token = Some(token_enc);
    User::update(id.clone(), &salted_user, &postgres_pool)?;
    Ok(user)
}
