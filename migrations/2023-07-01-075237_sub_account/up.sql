-- Your SQL goes here
CREATE TABLE sub_account(
    id SERIAL PRIMARY KEY,
    uuid uuid NOT NULL,
    name VARCHAR NOT NULL,
    mail_address VARCHAR NOT NULL,
    account_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    otp_secret VARCHAR NOT NULL
)