use crate::utils::model_manager::Model;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

// User model and its mutations
#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[primary_key("uuid")]
#[table_name = "users"]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub token: Option<String>,
}

#[derive(Debug, AsChangeset, Insertable, Clone, Serialize, Deserialize)]
#[table_name = "users"]
pub struct MutUser {
    pub uuid: Option<String>,
    pub username: String,
    pub password: Option<String>,
    pub token: Option<String>,
}

impl Model<User, MutUser, String> for User {
    // Create a new user
    fn create(new: &MutUser, db: &diesel::PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(users::table).values(new).execute(db)
    }

    // Read a user by username
    fn read_one(id: String, db: &diesel::PgConnection) -> Result<User, diesel::result::Error> {
        use users::dsl::username;

        Ok(users::table.filter(username.eq(id)).first::<User>(db)?)
    }

    // Read all users
    fn read_all(_: &diesel::PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        unimplemented!()
    }

    // Update a user by username
    fn update(
        id: String,
        new: &MutUser,
        db: &diesel::PgConnection,
    ) -> Result<usize, diesel::result::Error> {
        use users::dsl::username;
        let update = diesel::update(users::table.filter(username.eq(id)))
            .set(new)
            .execute(db)?;

        Ok(update)
    }

    // Delete a user by username
    fn delete(_: String, _: &diesel::PgConnection) -> Result<usize, diesel::result::Error> {
        todo!()
    }
}

impl User {
    // Read a user by token

    pub fn update_with_token(
        new: &MutUser,
        db: &diesel::PgConnection,
    ) -> Result<usize, diesel::result::Error> {
        use users::dsl::username;

        let res = diesel::update(users::table.filter(username.eq(new.username.clone())))
            .set(new)
            .execute(db)?;

        Ok(res)
    }
}
