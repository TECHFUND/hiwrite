use crate::user::model::MutUser;
use crate::utils::auth::encrypt;
use crate::utils::auth::Claims;
use crate::utils::error::CustomHttpError;
use actix_web::cookie::Cookie;
use time::Duration;
use time::OffsetDateTime;

pub(crate) fn login_res(user: &mut MutUser) -> Result<Cookie, CustomHttpError> {
    let claim = Claims {
        exp: (chrono::Utc::now() + chrono::Duration::days(10)).timestamp() as usize,
        sub: user.username.clone(),
    };
    user.password = None;
    let token_enc = encrypt(claim)?;
    let time: OffsetDateTime = OffsetDateTime::now_utc() + Duration::hour();
    let cookie = Cookie::build("auth", token_enc)
        .expires(time)
        .path("/")
        .finish();
    Ok(cookie)
}
