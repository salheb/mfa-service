use chrono::{NaiveDateTime};
use diesel::{AsChangeset, Queryable, Identifiable};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::{Uuid};

#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::core::app::schema::sub_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SubAccountEntity{
    id: i32,
    uuid: Uuid,
    name: String,
    mail_address: String,
    account_id: i32,
    created_at: NaiveDateTime
}

impl SubAccountEntity {
    pub fn new(
        id: i32,
        uuid: Uuid,
        name: String,
        mail_address: String,
        account_id: i32,
        created_at: NaiveDateTime) -> SubAccountEntity{

                SubAccountEntity { id, uuid , name , mail_address , account_id , created_at }
    }
    
}

#[cfg(test)]
mod tests{
    use uuid::Uuid;

    use super::{SubAccountEntity};
    use chrono::Utc;

    #[test]
    fn should_create_sub_account_entity(){
        let sub_account: SubAccountEntity = SubAccountEntity  {  
                id: 30,
                uuid: Uuid::new_v4(),  
                name: "Internal Sector".to_string(), 
                mail_address: "test@mail.org".to_string(), 
                account_id: 1,
                created_at: Utc::now().naive_utc()
            };
        assert_eq!(sub_account.account_id,1);
        assert_eq!(sub_account.mail_address as String, "test@mail.org");
    }
}