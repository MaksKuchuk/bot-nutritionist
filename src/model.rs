use diesel::prelude::*;
pub mod schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::model::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub gender: i32,
    pub age: i32,
    pub height: i32,
    pub weight: i32,
    pub physical_activity_level: i32,
    pub goal: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::model::schema::genders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Gender {
    pub id: i32,
    pub gender: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::model::schema::physicalactivitylevels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PhysicalActivityLevel {
    pub id: i32,
    pub physical_activity_level: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::model::schema::goals)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Goal {
    pub id: i32,
    pub goal: String,
}
