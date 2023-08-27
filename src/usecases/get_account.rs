use diesel::result::Error;
use crate::{domain::account::Account, adapters::postgres};


pub async fn get_account(account_id: i32) -> Result<Account, Error>{
       
    let entity = postgres::get_account(account_id).await;
    let model = match entity {
        Ok(account) => Account::from_entity(account),
        Err(e) => return Err(e),
    };
    
    Ok(model)
}