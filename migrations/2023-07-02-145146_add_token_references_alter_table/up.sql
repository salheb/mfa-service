-- Your SQL goes here
ALTER TABLE token 
ADD CONSTRAINT fk_token_account 
FOREIGN KEY (account) 
REFERENCES account(id);

ALTER TABLE token 
ADD CONSTRAINT fk_token_sub_account 
FOREIGN KEY (sub_account) 
REFERENCES sub_account(id);

ALTER TABLE sub_account
ADD CONSTRAINT fk_account_sub_account
FOREIGN KEY (account_id)
REFERENCES account(id);