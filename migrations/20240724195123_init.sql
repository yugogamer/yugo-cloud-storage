-- Add migration script here
BEGIN;

CREATE TABLE user_accounts(
    user_id BIGSERIAL primary key,
    username TEXT not null,
    email text not null check (length(email) < 255),
    create_date timestamp with time zone not null default NOW(),
    UNIQUE(email)
);

CREATE TABLE user_auths(
    user_id BIGINT primary key references user_accounts(user_id),
    password TEXT not null
);

COMMIT;