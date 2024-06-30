use diesel::prelude::*;

use crate::schema::{ChoosenDiets, DietExamples, Foods, UserDiets, Users};
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

#[derive(Queryable, Selectable, Debug, QueryableByName, Clone)]
#[diesel(table_name = DietExamples)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DietExample {
    pub id: i32,
    pub products: String,
    pub weights: String,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = UserDiets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserDiet {
    pub id: i32,
    pub userid: String,
    pub name: String,
    pub diet: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = UserDiets)]
pub struct NewUserDiet {
    pub userid: String,
    pub name: String,
    pub diet: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = ChoosenDiets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ChoosenDiet {
    pub userid: String,
    pub dietid: i32,
    pub state: i32,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = ChoosenDiets)]
pub struct NewChoosenDiet {
    pub userid: String,
    pub dietid: i32,
    pub state: i32,
}
