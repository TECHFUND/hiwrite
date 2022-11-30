use actix_web::Scope;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct LocalConfig {
    pub postgres_username: String,
    pub postgres_password: String,
    pub postgres_database: String,
    pub postgres_url: Option<String>,
    pub postgres_port: Option<u16>,
    pub bind_address: String,
    pub bind_port: u16,
    pub socket_dir: Option<String>,
    pub sql_name: Option<String>,
    pub max_req: u16,
    pub jwt_key: String,
}
pub trait Router {
    fn new() -> Scope;
}
