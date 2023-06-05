// @generated automatically by Diesel CLI.

diesel::table! {
    sprint (id) {
        id -> Integer,
        name -> Text,
        start -> Date,
        end -> Date,
    }
}

diesel::table! {
    task (id) {
        id -> Integer,
        name -> Text,
        sprint -> Integer,
        ordinal -> SmallInt,
        developer -> Integer,
        sp -> Double,
        tester -> Nullable<Integer>,
        test_sp -> Nullable<Double>,
        start -> Nullable<Timestamp>,
        end -> Nullable<Timestamp>,
        test_start -> Nullable<Timestamp>,
        test_end -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Text,
        role -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sprint,
    task,
    user,
);
