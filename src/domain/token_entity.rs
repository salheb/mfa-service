use chrono::{NaiveDateTime};
use diesel::{AsChangeset, Queryable, Identifiable};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::{Uuid};

#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::core::app::schema::token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(AccountEntity))]
#[diesel(belongs_to(SubAccountEntity))]
pub struct TokenEntity{
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

#[derive(ToSchema, Copy, Clone, Serialize, Deserialize)]
pub enum ChallengeType{
    SMS = 1,
    MAIL = 2,
    API = 3,
    WHATSAPP = 4
}

impl TokenEntity{
    pub fn new(id: i64, uuid: Uuid, account: i32, challenge_type: i32, sub_account: i32, 
                phone_number: String, mail_address: String, ttl: i32, 
                length: i32, created_at: NaiveDateTime) -> TokenEntity{
                    TokenEntity {id, uuid , account, challenge_type, sub_account, phone_number, mail_address, ttl, length, created_at}
                }

}