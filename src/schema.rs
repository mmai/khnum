table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        login -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        active -> Bool,
        expires_at -> Nullable<Timestamp>,
    }
}
