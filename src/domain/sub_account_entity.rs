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