//! The database models.

use bcrypt::{hash, DEFAULT_COST};

use schema::users;

/// A user.
#[derive(Clone, Debug, Queryable)]
pub struct User {
    /// The user's database ID.
    pub id: i32,

    /// The user's login name.
    pub username: String,

    /// The user's password hash.
    pub passhash: String,
}

/// A user to be added to the database.
#[derive(Debug, Insertable)]
#[table_name = "users"]
pub(crate) struct NewUser<'a> {
    /// The username to insert.
    username: &'a str,

    /// The password hash to insert.
    passhash: String,
}

impl<'a> NewUser<'a> {
    pub fn new<'b>(username: &'a str, password: &'b str) -> NewUser<'a> {
        let passhash = hash(password, DEFAULT_COST).expect("Failed to hash password");
        NewUser { username, passhash }
    }
}
