pub struct TokenEntity{
    id: i32,
    client_id: String,
    challenge_type: i32,
    sub_account: i32,
    phone_number: i32,
    mail_address: String,
    ttl: i32,
    length: i32,
    created_at: DateTime 
}

pub enum Challenge_Type{
    SMS = 1,
    MAIL = 2,
    API = 3,
    WHATSAPP = 4
}

impl TokenEntity{
    pub fn new(id: i32, client_id: i32, challenge_type: String, sub_account: i32, 
                phone_number: i32, mail_address: String, ttl: i32, 
                length: i32, created_at: DateTime){
                    TokenEntity {id , client_id, challenge_type, sub_account, phone_number, mail_address, ttl, length, created_at}
                }

}