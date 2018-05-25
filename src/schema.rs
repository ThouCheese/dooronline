table! {
    log (id) {
        id -> Int4,
        user_id -> Int4,
        date -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        is_admin -> Bool,
    }
}

joinable!(log -> user (user_id));

allow_tables_to_appear_in_same_query!(
    log,
    user,
);
