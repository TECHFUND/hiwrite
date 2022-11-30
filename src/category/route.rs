use super::controller::{create::*, delete::*, get::*, update::*};
use crate::utils::structs::Router;
use actix_web::{web, Scope};

pub struct CategoryRouter;

impl Router for CategoryRouter {
    fn new() -> Scope {
        web::scope("/category")
            .route("", web::post().to(create_controller))
            .route("/{id}", web::put().to(update_category))
            .route("/{id}", web::get().to(get_category))
            .route("/{id}", web::delete().to(delete_category))
    }
}
