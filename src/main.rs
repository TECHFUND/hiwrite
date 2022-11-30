use actix_cors::Cors;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::Connection;
use diesel::PgConnection;
use diesel_migrations::run_pending_migrations;
use dotenv::dotenv;
use envy;
use handlebars::Handlebars;
use std::sync::Mutex;
use std::time::Duration;

mod category;
mod module;
mod page;
mod schema;
mod user;
mod utils;

use crate::page::controller::display::display_page;
use crate::user::router::UserRouter;
use crate::utils::model_manager::*;
use crate::utils::structs::Router;
use category::route::CategoryRouter;
use module::router::ModuleRouter;
use page::router::PageRouter;
use utils::structs::LocalConfig;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    embed_migrations!();
    if cfg!(debug_assertions) {
        dotenv().unwrap();
    }

    let conf = envy::prefixed("app_").from_env::<LocalConfig>().unwrap();
    let pool = establish_database_connection(conf.clone()).unwrap();

    match run_pending_migrations(
        &PgConnection::establish(&format_connection_string(conf.clone())).unwrap(),
    ) {
        Ok(_) => println!("Ran migrations."),
        Err(_) => println!("Migrations not ran."),
    };

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let handlebars = Handlebars::new();

    let handlebars_ref = web::Data::new(Mutex::new(handlebars));
    let hb = handlebars_ref.clone();

    hb.lock()
        .unwrap()
        .register_templates_directory(".hbs", "./templates")
        .unwrap();

    utils::default::register_helpers(handlebars_ref.clone());

    let store = MemoryStore::new();

    let server_url = &format!("{}:{}", &conf.bind_address, &conf.bind_port);

    let http_server = HttpServer::new(move || {
        let cors = Cors::permissive();

        let api_scope = web::scope("/v1")
            .service(UserRouter::new())
            .service(PageRouter::new())
            .service(ModuleRouter::new())
            .service(CategoryRouter::new());

        let rate_limiting = RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
            .with_interval(Duration::from_secs(60))
            .with_max_requests(usize::from(conf.max_req));

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%a -> %U | %Dms "))
            .wrap(rate_limiting)
            .service(api_scope)
            .default_service(web::get().to(display_page))
            .data(pool.clone())
            .app_data(handlebars_ref.clone())
    })
    .bind(server_url)?
    .workers(2)
    .run();

    println!("HiWrite Up!");

    http_server.await
}
