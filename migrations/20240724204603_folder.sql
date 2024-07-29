-- Add migration script here
BEGIN;

CREATE TABLE folders(
    folder_id BIGSERIAL primary key,
    owner BIGINT references user_accounts(user_id) not null,
    name TEXT not null,
    is_home boolean not null default false,
    parent_folder BIGINT references folders(folder_id),

    create_date timestamp with time zone default NOW(),
    last_update timestamp with time zone default NOW(),
    last_user_update BIGINT references user_accounts(user_id)
);

CREATE TABLE folder_rights(
    read boolean not null default false,
    update boolean not null default false,

    user_id BIGINT references user_accounts(user_id),
    folder_id BIGINT references folders(folder_id),

    PRIMARY KEY(user_id, folder_id)
);



COMMIT;