-- Your SQL goes here
CREATE TABLE token(
    id BIGSERIAL PRIMARY KEY,
    uuid uuid NOT NULL,
    account INT NOT NULL,
    challenge_type INT NOT NULL,
    sub_account INT NOT NULL,
    phone_number VARCHAR NOT NULL,
    mail_address VARCHAR NOT NULL,
    ttl INT NOT NULL,
    length INT NOT NULL,
    created_at TIMESTAMP NOT NULL
)