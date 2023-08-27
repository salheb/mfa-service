use chrono::NaiveDateTime;
use diesel::{AsChangeset, Queryable, Identifiable, Selectable, Insertable};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize, ToSchema, Default, Selectable)]
#[diesel(table_name = crate::core::app::schema::token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(AccountEntity))]
#[diesel(belongs_to(SubAccountEntity))]
pub struct TokenEntity{
    pub id: i64,
    pub uuid: Uuid,
    pub account: i32,
    pub challenge_type: i32,
    pub sub_account: i32,
    pub phone_number: String,
    pub mail_address: String,
    pub ttl: i32,
    pub digits: i32,
    pub created_at: NaiveDateTime,
    pub code: String,
}

#[derive(ToSchema, Copy, Clone, Serialize, Deserialize)]
pub enum ChallengeType{
    Api = 1,
    Sms = 2,
    Mail = 3,
    Whatsapp = 4
}

impl ChallengeType{
    pub fn from_u32(value: u32) -> ChallengeType{
        match value {
            1 => ChallengeType::Api,
            2 => ChallengeType::Sms,
            3 => ChallengeType::Mail,
            4 => ChallengeType::Whatsapp,
            _ => panic!("Unknown Challenge Type enum conversion {}.", value),
        }
    }
}

impl TokenEntity{
    #[allow(clippy::too_many_arguments)]
    pub fn new(id: i64, uuid: Uuid, account: i32, challenge_type: i32, sub_account: i32, 
                phone_number: String, mail_address: String, ttl: i32, 
                digits: i32, created_at: NaiveDateTime, code: String) -> TokenEntity{
                    TokenEntity {id, uuid , account, challenge_type, sub_account, phone_number, mail_address, ttl, digits, created_at, code}
                }
    
    pub fn clone(&self) -> NewTokenEntity{
        NewTokenEntity { uuid: self.uuid, 
                         account: self.account, 
                         challenge_type: self.challenge_type, 
                         sub_account: self.sub_account, 
                         phone_number: self.phone_number.clone(), 
                         mail_address: self.mail_address.clone(), 
                         ttl: self.ttl, 
                         digits: self.digits, 
                         created_at: self.created_at,
                         code: self.code.clone()
                    }
    }
}

#[derive(Insertable, Selectable)]
#[diesel(table_name = crate::core::app::schema::token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(AccountEntity))]
#[diesel(belongs_to(SubAccountEntity))]
pub struct NewTokenEntity{
    pub uuid: Uuid,
    pub account: i32,
    pub challenge_type: i32,
    pub sub_account: i32,
    pub phone_number: String,
    pub mail_address: String,
    pub ttl: i32,
    pub digits: i32,
    pub created_at: NaiveDateTime,
    pub code: String
}

impl NewTokenEntity{
    #[allow(clippy::too_many_arguments)]
    pub fn new(uuid: Uuid, account: i32, challenge_type: i32, sub_account: i32, 
                phone_number: String, mail_address: String, ttl: i32, 
                digits: i32, created_at: NaiveDateTime, code: String) -> NewTokenEntity{
                    NewTokenEntity {uuid , account, challenge_type, sub_account, phone_number, mail_address, ttl, digits, created_at, code}
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
                challenge_type: ChallengeType::Api as i32, 
                sub_account: 1, 
                phone_number: "+5511940041111".to_string(), 
                mail_address: "test@mail.org".to_string(), 
                ttl: 60000, 
                digits: 6, 
                created_at: Utc::now().naive_utc(),
                code: "123456".to_string(),
            };
        assert_eq!(token.id,1);
        assert_eq!(token.challenge_type, 1);
    }
}