use crate::{domain::account::Account, adapters::postgres};


pub async fn create_account(account: Account) -> Account{
    let mut entity = Account::to_entity(account);

    entity = postgres::create_account(entity).await.unwrap();

    Account::from_entity(entity)
}