diesel::table! {
    users (id) {
        id -> Int4,
        gender -> Int4,
        age -> Int4,
        height -> Int4,
        weight -> Int4,
        physical_activity_level -> Int4,
        goal -> Int4,
    }
}

diesel::table! {
    genders (id) {
        id -> Int4,
        gender -> Text,
    }
}

diesel::table! {
    physicalactivitylevels (id) {
        id -> Int4,
        physical_activity_level -> Text,
    }
}

diesel::table! {
    goals (id) {
        id -> Int4,
        goal -> Text,
    }
}
