use diesel::{RunQueryDsl, result::Error, QueryDsl, ExpressionMethods};

use crate::core::app::schema::*;

use self::{entity::{account_entity::AccountEntity, sub_account_entity::SubAccountEntity}};

pub mod postgres_connection;
pub mod entity;

/*
CREATE ACCOUNT DATABASE METHOD
 */
pub async fn create_account(account_entity: AccountEntity) -> Result<AccountEntity, Error>{

    let pool = postgres_connection::get_pool();
    let conn = &mut pool.get().unwrap();
    
    diesel::insert_into(account::table)
            .values(account_entity.clone())
            .get_result::<AccountEntity>(conn)

}

/*
GET ACCOUNT DATABASE METHOD
 */
pub async fn get_account(account_id: i32) -> Result<AccountEntity, Error>{
    use crate::core::app::schema::account::dsl::*;

    let pool = postgres_connection::get_pool();
    let conn = &mut pool.get().unwrap();

    let selected_account = 
                    QueryDsl::filter(account, id.eq(account_id))
                        .first::<AccountEntity>(conn);

    return selected_account;
}

/*
CREATE ACCOUNT DATABASE METHOD
 */
pub async fn create_sub_account(sub_account_entity: SubAccountEntity) -> Result<SubAccountEntity, Error>{
    let pool = postgres_connection::get_pool();
    let conn = &mut pool.get().unwrap();

    diesel::insert_into(sub_account::table)
    .values(sub_account_entity.clone())
    .get_result::<SubAccountEntity>(conn)
}