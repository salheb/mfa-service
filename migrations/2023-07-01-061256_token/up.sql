-- Your SQL goes here
CREATE TABLE token(
    id BIGSERIAL PRIMARY KEY,
    uuid uuid,
    account INT,
    challenge_type INT,
    sub_account INT,
    phone_number VARCHAR,
    mail_address VARCHAR,
    ttl INT,
    length INT,
    created_at TIMESTAMP NOT NULL
)