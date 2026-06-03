use crate::user::model::MutUser;
use crate::user::model::User;
use crate::utils::auth::encrypt_password;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::Model;
use crate::utils::model_manager::PGPool;
use crate::utils::model_manager::pool_handler;
use actix_web::HttpResponse;
use actix_web::web;
use uuid::Uuid;

pub async fn signup(
    user: web::Json<MutUser>,
    pool: web::Data<PGPool>,
) -> Result<HttpResponse, HttpErrorCodes> {
    let mut postgres_pool = pool_handler(pool)?;

    if let Ok(_) = User::read_one(user.username.clone(), &mut postgres_pool) {
        Ok(HttpResponse::BadRequest().json("User already exists."))
    } else {
        let mut salted_user = user.clone();
        let encrypted_password = encrypt_password(&salted_user.password.unwrap())?;
        salted_user.password = Some(encrypted_password);
        salted_user.uuid = Some(Uuid::new_v4().to_string());

        User::create(&salted_user, &mut postgres_pool)?;

        Ok(HttpResponse::Created().json("You successfully signed up"))
    }
}
