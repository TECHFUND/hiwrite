use actix_web::web;
use diesel::{
    query_builder::AsChangeset,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    PgConnection,
};

use crate::utils::error::HttpErrorCodes;

use super::structs::LocalConfig;

pub type PGPool = Pool<ConnectionManager<PgConnection>>;
pub type PGPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait Model<TQueryable, TMutable: AsChangeset, TPrimary, TDto = TQueryable> {
    fn create(new: &TMutable, db: &PgConnection) -> Result<usize, diesel::result::Error>;
    fn read_one(id: TPrimary, db: &PgConnection) -> Result<TDto, diesel::result::Error>;
    fn read_all(db: &PgConnection) -> Result<Vec<TDto>, diesel::result::Error>;
    fn update(
        id: TPrimary,
        new: &TMutable,
        db: &PgConnection,
    ) -> Result<usize, diesel::result::Error>;
    fn delete(id: TPrimary, db: &PgConnection) -> Result<usize, diesel::result::Error>;
}

pub trait DTO<TColumns> {
    fn columns() -> TColumns;
}

pub trait Joinable<TLeft, TRight, TPrimary> {
    fn read_one_join_on(
        id: TPrimary,
        db: &PgConnection,
    ) -> Result<(TLeft, Vec<TRight>), diesel::result::Error>;
}

pub fn format_connection_string(conf: LocalConfig) -> String {
    match conf.postgres_url {
        Some(postgres_url) => {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                conf.postgres_username,
                conf.postgres_password,
                postgres_url,
                conf.postgres_port.unwrap(),
                conf.postgres_database
            )
        }
        None if std::env::var("PG_UNIX_PORT").is_ok() => {
            format!(
                "postgres://{}:{}@/{}",
                conf.postgres_username, conf.postgres_password, conf.postgres_database
            )
        }
        None => {
            panic!("Must supply one of the following: [postgres_url], [sql_name | socket_dir]")
        }
    }
}

pub fn establish_database_connection(conf: LocalConfig) -> Option<PGPool> {
    let db_url = format_connection_string(conf);

    Some(init_pool(&db_url).expect("Failed to create pool."))
}

pub fn init_connection(db_url: &str) -> ConnectionManager<diesel::PgConnection> {
    ConnectionManager::<PgConnection>::new(db_url)
}

pub fn init_pool(db_url: &str) -> Result<PGPool, PoolError> {
    let manager = init_connection(db_url);
    Pool::builder().max_size(2).build(manager)
}

pub fn pool_handler(pool: web::Data<PGPool>) -> Result<PGPooledConnection, HttpErrorCodes> {
    pool.get().or(Err(HttpErrorCodes::BadRequest))
}
