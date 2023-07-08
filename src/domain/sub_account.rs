use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct SubAccount{
    id: i32,
    uuid: Uuid,
    name: String,
    mail_address: String,
    account_id: i32,
    created_at: NaiveDateTime
}