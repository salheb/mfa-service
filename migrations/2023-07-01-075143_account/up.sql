-- Your SQL goes here
CREATE TABLE account(
    id SERIAL PRIMARY KEY,
    uuid uuid,
    account VARCHAR,
    mail_address VARCHAR,
    created_at DATETIME NOT NULL
)