use super::controller::{check::*, create::*, delete::*, get::*, login::*, logout::*, update::*};
use crate::utils::structs::Router;
use actix_web::{web, Scope};

pub struct UserRouter;

impl Router for UserRouter {
    fn new() -> Scope {
        web::scope("/user")
            .route("", web::post().to(create_user))
            .route("", web::get().to(check_login))
            .route("/login", web::post().to(login))
            .route("/logout", web::delete().to(logout))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::delete().to(delete_user))
    }
}
