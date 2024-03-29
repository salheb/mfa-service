use diesel::{RunQueryDsl, result::Error, QueryDsl, ExpressionMethods};

use crate::core::app::schema::*;

use self::entity::{account_entity::AccountEntity, sub_account_entity::SubAccountEntity, token_entity::TokenEntity};

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

    QueryDsl::filter(account, id.eq(account_id))
                        .first::<AccountEntity>(conn)
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

/*
GET ACCOUNT DATABASE METHOD
 */
pub async fn get_sub_account(sub_account_id: i32) -> Result<SubAccountEntity, Error>{
    use crate::core::app::schema::sub_account::dsl::*;

    let pool = postgres_connection::get_pool();
    let conn = &mut pool.get().unwrap();

    QueryDsl::filter(sub_account, id.eq(sub_account_id))
        .first::<SubAccountEntity>(conn)
}


/*
SAVE GENERATED OTP PASSWORD
 */
pub async fn save_generated_totp(token_entity: TokenEntity) -> Result<TokenEntity, Error>{
    let pool = postgres_connection::get_pool();
    let conn = &mut pool.get().unwrap();

    diesel::insert_into(token::table)
    .values(token_entity.clone())
    .get_result::<TokenEntity>(conn)
}

/*
VALIDATE OTP PASSWORD
 */
pub async fn get_totp(token_uuid: uuid::Uuid) -> Result<TokenEntity, Error>{
    use crate::core::app::schema::token::dsl::*;

    let pool = postgres_connection::get_pool();
    let conn = &mut pool.get().unwrap();

    QueryDsl::filter(token, uuid.eq(token_uuid))
                        .first::<TokenEntity>(conn)
}