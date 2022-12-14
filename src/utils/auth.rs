use actix_web::{dev::Payload, http::HeaderValue, web, FromRequest, HttpRequest};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::PgConnection;
use futures::{future::LocalBoxFuture, Future};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::error::CustomHttpError;
use crate::user::model;
use crate::utils::model_manager::{pool_handler, Model, PGPool};

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("An unknown cryptographic error has occured")]
    Unknown,
    #[error("User has failed their token comparison")]
    FailedComparison,
    #[error("There is no user present")]
    NoUser,
    #[error("The user is not logged in")]
    NotLoggedIn,
    #[error("No auth header present.")]
    NoAuthHeader,
    #[error("Password operation failed.")]
    OperationFail,
}

impl From<jsonwebtoken::errors::Error> for CryptoError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            _ => Self::Unknown,
        }
    }
}

impl From<argon2::password_hash::Error> for CryptoError {
    fn from(e: argon2::password_hash::Error) -> Self {
        match e {
            _ => Self::OperationFail,
        }
    }
}

pub fn encrypt(claim: Claims) -> Result<String, CryptoError> {
    let encoded_token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(std::env::var("APP_JWT_KEY").unwrap().as_bytes()),
    )?;

    Ok(encoded_token)
}

pub fn decrypt(jwt: &String) -> Result<Claims, CryptoError> {
    let decoded_token = decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(std::env::var("APP_JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )?;

    Ok(decoded_token.claims)
}

pub fn compare(token: &Claims, enc_token: &String, pool: &PgConnection) -> Result<(), CryptoError> {
    if let Ok(user) = model::User::read_one(token.sub.clone(), &pool) {
        if user.token.is_none() {
            return Err(CryptoError::NotLoggedIn);
        }
        if user.token == Some(enc_token.clone()) {
            return Ok(());
        } else {
            return Err(CryptoError::FailedComparison);
        };
    } else {
        return Err(CryptoError::NoUser);
    }
}

pub fn encrypt_password(password: &String) -> Result<String, CryptoError> {
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
    type Error = CustomHttpError;
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
            _ => Box::pin(async { Err(CryptoError::NoAuthHeader.into()) }),
        }
    }
}

pub fn authenticate(
    auth_header: &HeaderValue,
    db: &PgConnection,
) -> impl Future<Output = Result<Claims, CustomHttpError>> {
    let encrypted_token = std::str::from_utf8(auth_header.as_bytes())
        .unwrap()
        .to_string();

    let decrypted_token = decrypt(&encrypted_token);

    let mut logged_in = Err(CryptoError::NotLoggedIn);
    if let Ok(decrypted_token) = &decrypted_token {
        logged_in = compare(&decrypted_token, &encrypted_token, db);
    }

    async move {
        match logged_in {
            Ok(_) => Ok(decrypted_token?),
            Err(e) => Err(e.into()),
        }
    }
}
