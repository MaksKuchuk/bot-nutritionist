// @generated automatically by Diesel CLI.

diesel::table! {
    DietDishes (id) {
        id -> Integer,
        dietid -> Integer,
        datetime -> Text,
        dish -> Text,
        kcal -> Integer,
    }
}

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
    UserDiets (dietid) {
        dietid -> Integer,
        userid -> Text,
        name -> Text,
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

diesel::allow_tables_to_appear_in_same_query!(DietDishes, Foods, UserDiets, Users,);
