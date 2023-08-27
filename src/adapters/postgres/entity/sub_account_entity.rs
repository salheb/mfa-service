use chrono::NaiveDateTime;
use diesel::{AsChangeset, Queryable, Identifiable, Insertable, Selectable};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize, ToSchema, Default, Selectable)]
#[diesel(table_name = crate::core::app::schema::sub_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SubAccountEntity{
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub mail_address: String,
    pub account_id: i32,
    pub created_at: NaiveDateTime,
    pub otp_secret: String,
}

impl SubAccountEntity {
    pub fn new(
        id: i32,
        uuid: Uuid,
        name: String,
        mail_address: String,
        account_id: i32,
        created_at: NaiveDateTime,
        otp_secret: String) -> SubAccountEntity{

                SubAccountEntity { id, uuid , name , mail_address , account_id , created_at, otp_secret }
    }

    pub fn clone(&self) -> NewSubAccountEntity{
        NewSubAccountEntity { uuid: self.uuid, name: self.name.clone(), 
                              mail_address: self.mail_address.clone(), 
                              account_id: self.account_id, created_at: self.created_at,
                              otp_secret: self.otp_secret.clone()
                            }
    }
    
}

#[derive(Insertable, Selectable)]
#[diesel(table_name = crate::core::app::schema::sub_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSubAccountEntity{
    uuid: Uuid,
    name: String,
    mail_address: String,
    account_id: i32,
    created_at: NaiveDateTime,
    otp_secret: String
}

impl NewSubAccountEntity {
    pub fn new(
        uuid: Uuid,
        name: String,
        mail_address: String,
        account_id: i32,
        created_at: NaiveDateTime,
        otp_secret: String) -> NewSubAccountEntity{

            NewSubAccountEntity { uuid , name , mail_address , account_id , created_at, otp_secret }
    }
    
}

#[cfg(test)]
mod tests{
    use uuid::Uuid;

    use crate::adapters::postgres::entity::sub_account_entity::NewSubAccountEntity;

    use super::SubAccountEntity;
    use chrono::Utc;

    #[test]
    fn should_create_sub_account_entity(){
        let sub_account: SubAccountEntity = SubAccountEntity  {  
                id: 30,
                uuid: Uuid::new_v4(),  
                name: "Internal Sector".to_string(), 
                mail_address: "test@mail.org".to_string(), 
                account_id: 1,
                created_at: Utc::now().naive_utc(),
                otp_secret: "MyCustomSecret".to_string()
            };
        assert_eq!(sub_account.account_id,1);
        assert_eq!(sub_account.mail_address as String, "test@mail.org");
    }

    #[test]
    fn should_create_new_sub_account_entity(){
        let sub_account: NewSubAccountEntity = NewSubAccountEntity  {  
                uuid: Uuid::new_v4(),  
                name: "Internal Sector".to_string(), 
                mail_address: "test@mail.org".to_string(), 
                account_id: 1,
                created_at: Utc::now().naive_utc(),
                otp_secret: "MyCustomSecret".to_string()
            };
        assert_eq!(sub_account.account_id,1);
        assert_eq!(sub_account.mail_address as String, "test@mail.org");
    }

    #[test]
    fn should_clone_sub_account_entity(){
        let sub_account: SubAccountEntity = SubAccountEntity  {  
                id: 30,
                uuid: Uuid::new_v4(),  
                name: "Internal Sector".to_string(), 
                mail_address: "test@mail.org".to_string(), 
                account_id: 1,
                created_at: Utc::now().naive_utc(),
                otp_secret: "MyCustomSecret".to_string()
            };
        let new_sub_account: NewSubAccountEntity = sub_account.clone();
        assert_eq!(new_sub_account.account_id,1);
        assert_eq!(new_sub_account.mail_address as String, "test@mail.org");
    }


}