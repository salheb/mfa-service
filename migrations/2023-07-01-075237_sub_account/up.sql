-- Your SQL goes here
CREATE TABLE sub_account(
    id SERIAL PRIMARY KEY,
    sub_account VARCHAR,
    mail_address VARCHAR,
    account_id INTEGER,
    created_at DATETIME NOT NULL,
    CONSTRAINT fk_account
        FOREIGN KEY(account_id)
            REFERENCES account(id)
)