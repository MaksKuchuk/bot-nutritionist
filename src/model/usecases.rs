use diesel::query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl};
use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper, SqliteConnection};

use crate::model::{Food, NewUser, User};

pub fn create_update_user(conn: &mut SqliteConnection, user: NewUser) -> bool {
    use crate::schema::Users::dsl::*;

    match diesel::insert_into(Users).values(&user).execute(conn) {
        Ok(_) => true,
        _ => match diesel::update(Users)
            .filter(id.eq(&user.id))
            .set(&user)
            .execute(conn)
        {
            Ok(_) => true,
            _ => false,
        },
    }
}

pub fn get_user(conn: &mut SqliteConnection, uid: String) -> Option<User> {
    use crate::schema::Users::dsl::*;

    match Users.filter(id.eq(uid)).first::<User>(conn) {
        Ok(u) => Some(u),
        Err(_) => None,
    }
}

pub fn get_food_by_category(
    conn: &mut SqliteConnection,
    cat: &str,
    amount: u32,
) -> Option<Vec<Food>> {
    use crate::schema::Foods::dsl::*;

    match Foods
        .filter(category.eq(cat))
        .limit(amount as i64)
        .select(Food::as_select())
        .load(conn)
    {
        Ok(f) => Some(f),
        _ => None,
    }
}
