-- Your SQL goes here
create table user (
    id char(26),
    username varchar(32) not null,

    CONSTRAINT user_id_pk PRIMARY KEY (id),
    CONSTRAINT user_username_uq UNIQUE (username)
)