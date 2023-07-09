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

#[cfg(test)]
mod tests{
    use uuid::Uuid;

    use super::{TokenEntity, ChallengeType};
    use chrono::Utc;

    #[test]
    fn should_create_token_entity(){
        let token: TokenEntity = TokenEntity { 
                id: 1, 
                uuid: Uuid::new_v4(), 
                account: 1, 
                challenge_type: ChallengeType::API as i32, 
                sub_account: 1, 
                phone_number: "+5511940041111".to_string(), 
                mail_address: "test@mail.org".to_string(), 
                ttl: 60000, 
                length: 6, 
                created_at: Utc::now().naive_utc(),
            };
        assert_eq!(token.id,1);
        assert_eq!(token.challenge_type as i32, 3);
    }
}