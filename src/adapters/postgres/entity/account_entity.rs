use chrono::{NaiveDateTime};
use diesel::{AsChangeset, Queryable, Identifiable, Insertable, Selectable};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::{Uuid};

#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize, ToSchema, Default, Selectable)]
#[diesel(table_name = crate::core::app::schema::account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[primary_key(id)]
pub struct AccountEntity{
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub mail_address: String,
    pub created_at: NaiveDateTime
}


impl AccountEntity {
    pub fn new(id: i32,
        uuid: Uuid,
        name: String,
        mail_address: String,
        created_at: NaiveDateTime) -> AccountEntity{
            AccountEntity {id, uuid, name, mail_address, created_at}
    }

    pub fn clone(&self) -> NewAccountEntity{
        NewAccountEntity { uuid: self.uuid, name: self.name.clone(), mail_address: self.mail_address.clone(), created_at: self.created_at }
    }
}

#[derive(Insertable, Selectable)]
#[diesel(table_name = crate::core::app::schema::account)]
pub struct NewAccountEntity{
    pub uuid: Uuid,
    pub name: String,
    pub mail_address: String,
    pub created_at: NaiveDateTime
}

impl NewAccountEntity {
    pub fn new(
        uuid: Uuid,
        name: String,
        mail_address: String,
        created_at: NaiveDateTime) -> NewAccountEntity{
            NewAccountEntity {uuid, name, mail_address, created_at}
    }
}

#[cfg(test)]
mod tests{
    use uuid::Uuid;

    use crate::adapters::postgres::entity::account_entity::NewAccountEntity;

    use super::{AccountEntity};
    use chrono::Utc;

    #[test]
    fn should_create_account_entity(){
        let account: AccountEntity = AccountEntity  {  
                id: 30,
                uuid: Uuid::new_v4(),  
                name: "Customer Name".to_string(), 
                mail_address: "test@mail.org".to_string(), 
                created_at: Utc::now().naive_utc()
            };
        assert_eq!(account.id,30);
        assert_eq!(account.mail_address as String, "test@mail.org");
    }

    #[test]
    fn should_create_new_account_entity(){
        let account: NewAccountEntity = NewAccountEntity  {  
                uuid: Uuid::new_v4(),  
                name: "Customer Name".to_string(), 
                mail_address: "test@mail.org".to_string(), 
                created_at: Utc::now().naive_utc()
            };
        assert_eq!(account.name, "Customer Name");
        assert_eq!(account.mail_address as String, "test@mail.org");
    }

    #[test]
    fn should_clone_struct_account(){
        let account: AccountEntity = AccountEntity  {  
            id: 30,
            uuid: Uuid::new_v4(),  
            name: "Customer Name".to_string(), 
            mail_address: "test@mail.org".to_string(), 
            created_at: Utc::now().naive_utc()
        };
        let new_account: NewAccountEntity = account.clone();
        assert_eq!(new_account.name, "Customer Name");
        assert_eq!(new_account.mail_address as String, "test@mail.org");
    }


}