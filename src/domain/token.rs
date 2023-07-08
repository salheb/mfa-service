use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct Token{
    id: i64,
    uuid: Uuid,
    account: i32,
    challenge_type: i32,
    sub_account: i32,
    phone_number: String,
    mail_address: String,
    ttl: i32,
    length: i32,
    created_at: NaiveDateTime
}

