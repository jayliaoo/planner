// @generated automatically by Diesel CLI.

diesel::table! {
    sprint (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        start -> Date,
        end -> Date,
    }
}

diesel::table! {
    task (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        sprint -> Int4,
        ordinal -> Int2,
        developer -> Int4,
        sp -> Numeric,
        tester -> Nullable<Int4>,
        test_sp -> Nullable<Numeric>,
        start -> Nullable<Timestamp>,
        end -> Nullable<Timestamp>,
        test_start -> Nullable<Timestamp>,
        test_end -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 4]
        role -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sprint,
    task,
    user,
);
