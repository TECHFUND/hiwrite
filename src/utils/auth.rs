use actix_web::{dev::Payload, http::HeaderValue, web, FromRequest, HttpRequest};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::PgConnection;
use futures::{future::LocalBoxFuture, Future};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::error::HttpErrorCodes;
use crate::user::model;
use crate::utils::model_manager::{pool_handler, Model, PGPool};

#[derive(Error, Debug)]
pub enum ErrorCodes {
    // #[error("1000")] : "1000" is the default error code
    #[error("1001")]
    Unknown,

    // #[error("1002")] : "1002" for failed comparison
    #[error("1002")]
    FailedComparison,

    // #[error("1003")] : "1003" for no user
    #[error("1003")]
    NoUser,

    // #[error("1004")] : "1004" for not logged in
    #[error("1004")]
    NotLoggedIn,

    // #[error("1005")] : "1005" for no auth header
    #[error("1005")]
    NoAuthHeader,

    // #[error("1006")] : "1006" for operation fail
    #[error("1006")]
    OperationFail,
}

impl From<jsonwebtoken::errors::Error> for ErrorCodes {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            _ => Self::Unknown,
        }
    }
}

impl From<argon2::password_hash::Error> for ErrorCodes {
    fn from(e: argon2::password_hash::Error) -> Self {
        match e {
            _ => Self::OperationFail,
        }
    }
}

pub fn encrypt(claim: Claims) -> Result<String, ErrorCodes> {
    let encoded_token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(std::env::var("APP_JWT_KEY").unwrap().as_bytes()),
    )?;

    Ok(encoded_token)
}

pub fn decrypt(jwt: &String) -> Result<Claims, ErrorCodes> {
    let decoded_token = decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(std::env::var("APP_JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )?;

    Ok(decoded_token.claims)
}

pub fn compare(token: &Claims, enc_token: &String, pool: &PgConnection) -> Result<(), ErrorCodes> {
    // Read user from database and compare token
    if let Ok(user) = model::User::read_one(token.sub.clone(), &pool) {
        if user.token.is_none() {
            return Err(ErrorCodes::NotLoggedIn);
        }
        if user.token == Some(enc_token.clone()) {
            return Ok(());
        } else {
            return Err(ErrorCodes::FailedComparison);
        };
    } else {
        return Err(ErrorCodes::NoUser);
    }
}

pub fn encrypt_password(password: &String) -> Result<String, ErrorCodes> {
    // Generate salt and hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    return Ok(argon2
        .hash_password_simple(password.as_bytes(), &salt)?
        .to_string());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
}

impl FromRequest for Claims {
    type Error = HttpErrorCodes;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<web::Data<PGPool>>().unwrap().to_owned();
        let postgres_pool = pool_handler(pool).unwrap();
        let auth_header = req.headers().get("Authorization");

        match auth_header {
            Some(auth) => {
                let fut = authenticate(auth, &postgres_pool);
                Box::pin(fut)
            }
            _ => Box::pin(async { Err(ErrorCodes::NoAuthHeader.into()) }),
        }
    }
}

pub fn authenticate(
    auth_header: &HeaderValue,
    db: &PgConnection,
) -> impl Future<Output = Result<Claims, HttpErrorCodes>> {
    // Decrypt token and compare with database
    let encrypted_token = std::str::from_utf8(auth_header.as_bytes())
        .unwrap()
        .to_string();

    let decrypted_token = decrypt(&encrypted_token);

    // Check if user is logged in
    let mut logged_in = Err(ErrorCodes::NotLoggedIn);
    if let Ok(decrypted_token) = &decrypted_token {
        logged_in = compare(&decrypted_token, &encrypted_token, db);
    }

    // Return decrypted token if logged in
    async move {
        match logged_in {
            Ok(_) => Ok(decrypted_token?),
            Err(e) => Err(e.into()),
        }
    }
}
