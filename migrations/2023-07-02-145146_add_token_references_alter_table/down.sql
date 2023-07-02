-- This file should undo anything in `up.sql`
ALTER TABLE token
DROP CONSTRAINT fk_token_account;

ALTER TABLE token
DROP CONSTRAINT fk_token_sub_account;