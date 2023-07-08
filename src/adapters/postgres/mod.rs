use diesel::{RunQueryDsl, result::Error};

use crate::core::app::schema::*;

use self::{entity::account_entity::AccountEntity};

pub mod postgres_connection;
pub mod entity;

pub async fn create_account(account_entity: AccountEntity) -> Result<AccountEntity, Error>{

    let pool = postgres_connection::get_pool();
    let conn = &mut pool.get().unwrap();
    
    diesel::insert_into(account::table)
            .values(account_entity)
            .get_result::<AccountEntity>(conn)

}