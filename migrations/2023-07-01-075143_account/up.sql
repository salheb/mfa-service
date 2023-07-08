-- Your SQL goes here
CREATE TABLE account(
    id SERIAL PRIMARY KEY,
    uuid uuid NOT NULL,
    name VARCHAR NOT NULL,
    mail_address VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL
)