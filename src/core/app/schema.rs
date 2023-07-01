// @generated automatically by Diesel CLI.

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
