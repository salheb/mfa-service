// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int4,
        uuid -> Nullable<Uuid>,
        name -> Nullable<Varchar>,
        mail_address -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    sub_account (id) {
        id -> Int4,
        uuid -> Nullable<Uuid>,
        name -> Nullable<Varchar>,
        mail_address -> Nullable<Varchar>,
        account_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    token (id) {
        id -> Int8,
        uuid -> Nullable<Uuid>,
        account -> Nullable<Int4>,
        challenge_type -> Nullable<Int4>,
        sub_account -> Nullable<Int4>,
        phone_number -> Nullable<Varchar>,
        mail_address -> Nullable<Varchar>,
        ttl -> Nullable<Int4>,
        length -> Nullable<Int4>,
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
