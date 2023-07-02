-- Your SQL goes here
CREATE TABLE sub_account(
    id SERIAL PRIMARY KEY,
    uuid uuid,
    name VARCHAR,
    mail_address VARCHAR,
    account_id INTEGER,
    created_at TIMESTAMP NOT NULL
)