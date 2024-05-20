use diesel::result::Error;
use diesel::{RunQueryDsl, SqliteConnection};

use crate::model::{NewUser, User};
use crate::schema::users::dsl::users;

pub fn create_user(conn: &mut SqliteConnection, user: NewUser) -> Result<usize, Error> {
    diesel::insert_into(users).values(&user).execute(conn)
}

pub fn get_user(conn: &mut SqliteConnection) -> Result<User, Error> {
    users.first::<User>(conn)
}
