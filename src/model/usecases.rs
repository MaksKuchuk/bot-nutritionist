use std::collections::HashSet;

use diesel::query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl};
use diesel::result::Error;
use diesel::{Connection, ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::model::{Food, NewNotificationsTime, NewUser, NotificationTime, User};
use crate::schema::UserDiets;
use crate::DbConnection;

use super::{ChoosenDiet, DietExample, NewChoosenDiet, NewUserDiet, UserDiet};

pub fn create_update_user(conn: &mut DbConnection, user: NewUser) -> Result<(), Error> {
    use crate::schema::Users::dsl::*;

    conn.transaction::<_, Error, _>(|connection| {
        match diesel::insert_into(Users).values(&user).execute(connection) {
            Ok(_) => Ok(()),
            _ => match diesel::update(Users)
                .filter(id.eq(&user.id))
                .set(&user)
                .execute(connection)
            {
                Ok(_) => Ok(()),
                _ => Err(Error::AlreadyInTransaction),
            },
        }
    })
}

pub fn create_update_userdiet(
    conn: &mut DbConnection,
    userdiet: NewChoosenDiet,
    v: Vec<String>,
) -> Result<(), Error> {
    conn.transaction::<_, Error, _>(|connection| {
        {
            use crate::schema::NotificationsTime::dsl::*;
            match diesel::delete(NotificationsTime.filter(userid.eq(&userdiet.userid)))
                .execute(connection)
            {
                Err(_) => (),
                _ => (),
            };

            if userdiet.state == 1 {
                for t in v {
                    match diesel::insert_into(NotificationsTime)
                        .values(NewNotificationsTime {
                            userid: userdiet.userid.clone(),
                            time: t,
                        })
                        .execute(connection)
                    {
                        Err(_) => (),
                        _ => (),
                    };
                }
            }
        }

        use crate::schema::ChoosenDiets::dsl::*;
        match diesel::insert_into(ChoosenDiets)
            .values(&userdiet)
            .execute(connection)
        {
            Ok(_) => Ok(()),
            _ => match diesel::update(ChoosenDiets)
                .filter(userid.eq(&userdiet.userid))
                .set(&userdiet)
                .execute(connection)
            {
                Ok(_) => Ok(()),
                _ => Err(Error::AlreadyInTransaction),
            },
        }
    })
}

pub fn create_diet(conn: &mut DbConnection, user_diet: NewUserDiet) -> bool {
    use crate::schema::UserDiets::dsl::*;

    match diesel::insert_into(UserDiets)
        .values(&user_diet)
        .execute(conn)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_random_example_diet(conn: &mut DbConnection) -> Option<DietExample> {
    match diesel::dsl::sql_query("SELECT * FROM DietExamples ORDER BY RANDOM()")
        .load::<DietExample>(conn)
    {
        Ok(d) => {
            if !d.is_empty() {
                Some(d[0].to_owned())
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn update_diet(conn: &mut DbConnection, user_diet: NewUserDiet) -> bool {
    use crate::schema::UserDiets::dsl::*;

    match diesel::update(UserDiets)
        .filter(userid.eq(&user_diet.userid))
        .filter(name.eq(&user_diet.name))
        .set(&user_diet)
        .execute(conn)
    {
        Ok(_) => true,
        _ => false,
    }
}

pub fn delete_diet(conn: &mut DbConnection, usrid: String, nm: String) -> bool {
    use crate::schema::UserDiets::dsl::*;

    match diesel::delete(UserDiets.filter(userid.eq(usrid)).filter(name.eq(nm))).execute(conn) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_diets_by_userid(conn: &mut DbConnection, usrid: String) -> Option<Vec<UserDiet>> {
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
    conn: &mut DbConnection,
    usrid: String,
    nm: String,
) -> Option<UserDiet> {
    use crate::schema::UserDiets::dsl::*;

    match UserDiets
        .filter(userid.eq(usrid))
        .filter(name.eq(nm))
        .select(UserDiet::as_select())
        .first(conn)
    {
        Ok(d) => Some(d),
        _ => None,
    }
}

pub fn get_choosen_diet(conn: &mut DbConnection, usrid: String) -> Option<ChoosenDiet> {
    use crate::schema::ChoosenDiets::dsl::*;

    match ChoosenDiets.filter(userid.eq(usrid)).first(conn) {
        Ok(d) => Some(d),
        _ => None,
    }
}

pub fn set_user_notification(
    conn: &mut DbConnection,
    usrid: String,
    st: i32,
    v: Vec<String>,
) -> bool {
    {
        use crate::schema::NotificationsTime::dsl::*;
        match diesel::delete(NotificationsTime.filter(userid.eq(&usrid))).execute(conn) {
            Err(_) => (),
            _ => (),
        };

        if st == 1 {
            for t in v {
                match diesel::insert_into(NotificationsTime)
                    .values(NewNotificationsTime {
                        userid: usrid.clone(),
                        time: t,
                    })
                    .execute(conn)
                {
                    Err(_) => (),
                    _ => (),
                };
            }
        }
    }

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

pub fn get_diet_by_id(conn: &mut DbConnection, did: i32) -> Option<UserDiet> {
    use crate::schema::UserDiets::dsl::*;

    match UserDiets
        .filter(id.eq(did))
        .select(UserDiet::as_select())
        .first(conn)
    {
        Ok(u) => Some(u),
        Err(_) => None,
    }
}

pub fn get_user(conn: &mut DbConnection, uid: String) -> Option<User> {
    use crate::schema::Users::dsl::*;

    match Users.filter(id.eq(uid)).first::<User>(conn) {
        Ok(u) => Some(u),
        Err(_) => None,
    }
}

pub fn get_food_by_category(conn: &mut DbConnection, cat: &str, amount: u32) -> Option<Vec<Food>> {
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

pub fn get_set_userid_by_time(conn: &mut DbConnection, t: &str) -> HashSet<String> {
    use crate::schema::NotificationsTime::dsl::*;

    let res: Vec<NotificationTime> = match NotificationsTime
        .filter(time.eq(t))
        .select(NotificationTime::as_select())
        .load(conn)
    {
        Ok(v) => v,
        Err(_) => vec![],
    };

    let mut v: HashSet<String> = HashSet::new();

    for nt in res {
        v.insert(nt.userid);
    }

    v
}
