// @generated automatically by Diesel CLI.

diesel::table! {
    friend_requests (id) {
        id -> Uuid,
        from -> Uuid,
        to -> Uuid,
        is_accepted -> Bool,
        sent_at -> Timestamp,
        accepted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 16]
        name -> Varchar,
        #[max_length = 56]
        email -> Varchar,
        password -> Varchar,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    friend_requests,
    users,
);
