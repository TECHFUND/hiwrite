use super::parse::parse_page;
use crate::page::model::Page;
use crate::utils::error::HttpErrorCodes;
use crate::utils::model_manager::pool_handler;
use crate::utils::model_manager::PGPool;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use std::sync::Mutex;

pub async fn display_page(
    req: web::HttpRequest,
    pool: web::Data<PGPool>,
    hb: web::Data<Mutex<Handlebars<'_>>>,
) -> Result<HttpResponse, HttpErrorCodes> {
    // Postgres pool handler
    let postgres_pool = pool_handler(pool)?;
    // Get page
    let path = req.path();
    let page_tuple = Page::read_one_join_on_url(path.to_string(), &postgres_pool);
    if let Err(_) = page_tuple {
        let s = hb.lock().unwrap().render("404", &String::from("")).unwrap();
        return Ok(HttpResponse::Ok().content_type("text/html").body(s));
    }

    // parse page and render
    let pagemodule = parse_page(page_tuple?)?;
    let s = hb
        .lock()
        .unwrap()
        .render(&pagemodule.page_name, &pagemodule)
        .unwrap();

    // Return page
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
