use diesel::query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl};
use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper, SqliteConnection};

use crate::model::{Food, NewUser, User};

use super::{ChoosenDiet, NewChoosenDiet, NewUserDiet, UserDiet};

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

pub fn create_update_userdiet(conn: &mut SqliteConnection, userdiet: NewChoosenDiet) -> bool {
    use crate::schema::ChoosenDiets::dsl::*;

    match diesel::insert_into(ChoosenDiets)
        .values(&userdiet)
        .execute(conn)
    {
        Ok(_) => true,
        _ => match diesel::update(ChoosenDiets)
            .filter(userid.eq(&userdiet.userid))
            .set(&userdiet)
            .execute(conn)
        {
            Ok(_) => true,
            _ => false,
        },
    }
}

pub fn create_diet(conn: &mut SqliteConnection, user_diet: NewUserDiet) -> bool {
    use crate::schema::UserDiets::dsl::*;

    match diesel::insert_into(UserDiets)
        .values(&user_diet)
        .execute(conn)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn delete_diet(conn: &mut SqliteConnection, usrid: String, nm: String) -> bool {
    use crate::schema::UserDiets::dsl::*;

    match diesel::delete(UserDiets.filter(userid.eq(usrid)).filter(name.eq(nm))).execute(conn) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_diets_by_userid(conn: &mut SqliteConnection, usrid: String) -> Option<Vec<UserDiet>> {
    use crate::schema::UserDiets::dsl::*;

    match UserDiets
        .filter(userid.eq(usrid))
        .select(UserDiet::as_select())
        .load(conn)
    {
        Ok(d) => Some(d),
        _ => None,
    }
}

pub fn get_diets_by_userid_name(
    conn: &mut SqliteConnection,
    usrid: String,
    nm: String,
) -> Option<UserDiet> {
    use crate::schema::UserDiets::dsl::*;

    match UserDiets
        .filter(userid.eq(usrid))
        .filter(name.eq(nm))
        .first(conn)
    {
        Ok(d) => Some(d),
        _ => None,
    }
}

pub fn get_choosen_diet(conn: &mut SqliteConnection, usrid: String) -> Option<ChoosenDiet> {
    use crate::schema::ChoosenDiets::dsl::*;

    match ChoosenDiets.filter(userid.eq(usrid)).first(conn) {
        Ok(d) => Some(d),
        _ => None,
    }
}

pub fn set_user_notification(conn: &mut SqliteConnection, usrid: String, st: i32) -> bool {
    use crate::schema::ChoosenDiets::dsl::*;

    match diesel::update(ChoosenDiets)
        .filter(userid.eq(usrid))
        .set(state.eq(st))
        .execute(conn)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_diet_by_id(conn: &mut SqliteConnection, did: i32) -> Option<UserDiet> {
    use crate::schema::UserDiets::dsl::*;

    match UserDiets.filter(dietid.eq(did)).first(conn) {
        Ok(u) => Some(u),
        Err(_) => None,
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
