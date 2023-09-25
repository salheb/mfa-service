use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message{
    pub account: i32,
    // API = 1, SMS = 2, MAIL = 3, WHATSAPP = 4
    pub destination_type: i32,
    pub phone_number: String,
    pub mail_address: String,
    pub content: String,
}

impl Message{
    pub fn new(account: i32, destination_type: i32, phone_number: String, mail_address: String, content: String) -> Self{
        Self {account, destination_type, phone_number, mail_address, content}
    }

    pub fn to_bytes(self) -> Vec<u8>{
        bincode::serialize(&self).unwrap()
    }
}