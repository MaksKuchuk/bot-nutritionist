use diesel::prelude::*;

use crate::schema::{Foods, UserDiets, Users};
pub mod usecases;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = Users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub gender: String,
    pub age: i32,
    pub height: i32,
    pub weight: i32,
    pub physical_activity_level: String,
    pub goal: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = Users)]
pub struct NewUser {
    pub id: String,
    pub gender: String,
    pub age: i32,
    pub height: i32,
    pub weight: i32,
    pub physical_activity_level: String,
    pub goal: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = Foods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub kcal: i32,
    pub protein: i32,
    pub fat: i32,
    pub carbohydrate: i32,
    pub category: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = UserDiets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserDiet {
    pub dietid: i32,
    pub userid: String,
    pub name: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = UserDiets)]
pub struct NewUserDiet {
    pub userid: String,
    pub name: String,
}
