use crate::utils::error::HttpErrorCodes;
use actix_web::{cookie::Cookie, HttpResponse};
use time::OffsetDateTime;

pub async fn logout() -> Result<HttpResponse, HttpErrorCodes> {
    // Get cookie and set it to expire
    let cookie = Cookie::build("auth", "")
        .expires(OffsetDateTime::now_utc())
        .path("/")
        .finish();

    // Return cookie
    Ok(HttpResponse::Ok().cookie(cookie).finish())
}
