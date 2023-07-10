use chrono::Utc;
use uuid::Uuid;

use crate::{domain::account::Account, adapters::postgres};


pub async fn create_account(account: &mut Account) -> Account{
    
    *account.created_at() = Utc::now().naive_utc();
    *account.uuid() = Uuid::new_v4();

    let mut entity = Account::to_entity(account);
    
    entity = postgres::create_account(entity).await.unwrap();

    Account::from_entity(entity)
}