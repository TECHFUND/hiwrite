use crate::utils::error::CustomHttpError;
use actix_web::{cookie::Cookie, HttpResponse};
use time::OffsetDateTime;

pub async fn logout() -> Result<HttpResponse, CustomHttpError> {
    let cookie = Cookie::build("auth", "")
        .expires(OffsetDateTime::now_utc())
        .path("/")
        .finish();
    Ok(HttpResponse::Ok().cookie(cookie).finish())
}
