-- Your SQL goes here
CREATE TABLE account(
    id SERIAL PRIMARY KEY,
    uuid uuid,
    name VARCHAR,
    mail_address VARCHAR,
    created_at TIMESTAMP NOT NULL
)