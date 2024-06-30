// @generated automatically by Diesel CLI.

diesel::table! {
    ChoosenDiets (userid) {
        userid -> Text,
        dietid -> Integer,
        state -> Integer,
    }
}

diesel::table! {
    DietExamples (id) {
        id -> Integer,
        products -> Text,
        weights -> Text,
        proteins -> Integer,
        fats -> Integer,
        carbohydrates -> Integer,
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
    UserDiets (id) {
        id -> Integer,
        userid -> Text,
        name -> Text,
        diet -> Text,
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

diesel::allow_tables_to_appear_in_same_query!(
    ChoosenDiets,
    DietExamples,
    Foods,
    UserDiets,
    Users,
);
