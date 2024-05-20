// @generated automatically by Diesel CLI.

diesel::table! {
    genders (id) {
        id -> Integer,
        gender -> Text,
    }
}

diesel::table! {
    goals (id) {
        id -> Integer,
        goal -> Text,
    }
}

diesel::table! {
    physical_activity_levels (id) {
        id -> Integer,
        physical_activity_level -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        gender -> Integer,
        age -> Integer,
        height -> Integer,
        weight -> Integer,
        physical_activity_level -> Integer,
        goal -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(genders, goals, physical_activity_levels, users);
