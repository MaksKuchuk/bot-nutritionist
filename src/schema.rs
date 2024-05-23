// @generated automatically by Diesel CLI.

diesel::table! {
    Foods (id) {
        id -> Integer,
        name -> Text,
        kcal -> Integer,
        protein -> Integer,
        fat -> Integer,
        carbohydrate -> Integer,
        category -> Text,
    }
}

diesel::table! {
    Users (id) {
        id -> Text,
        gender -> Text,
        age -> Integer,
        height -> Integer,
        weight -> Integer,
        physical_activity_level -> Text,
        goal -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(Foods, Users,);
