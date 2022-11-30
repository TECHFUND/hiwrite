use crate::user::model::MutUser;
use crate::user::model::User;
use crate::utils::auth::encrypt_password;
use crate::utils::auth::Claims;
use crate::utils::error::CustomHttpError;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use actix_web::web;
use actix_web::HttpResponse;
use uuid::Uuid;

pub async fn create_user(
    new: web::Json<MutUser>,
    pool: web::Data<PGPool>,
    _: Claims,
) -> Result<HttpResponse, CustomHttpError> {
    let postgres_pool = pool_handler(pool)?;
    let mut salted_user = new.clone();
    let encrypted_password = encrypt_password(&salted_user.password.unwrap())?;
    salted_user.password = Some(encrypted_password);
    salted_user.uuid = Some(Uuid::new_v4().to_string());
    User::create(&salted_user, &postgres_pool)?;
    Ok(HttpResponse::Created().json(&new.clone()))
}
