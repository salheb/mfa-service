// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int4,
        uuid -> Uuid,
        name -> Varchar,
        mail_address -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    sub_account (id) {
        id -> Int4,
        uuid -> Uuid,
        name -> Varchar,
        mail_address -> Varchar,
        account_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    token (id) {
        id -> Int8,
        uuid -> Uuid,
        account -> Int4,
        challenge_type -> Int4,
        sub_account -> Int4,
        phone_number -> Varchar,
        mail_address -> Varchar,
        ttl -> Int4,
        length -> Int4,
        created_at -> Timestamp,
    }
}

diesel::joinable!(token -> account (account));
diesel::joinable!(token -> sub_account (sub_account));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    sub_account,
    token,
);
