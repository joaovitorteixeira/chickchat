-- Your SQL goes here
create table user (
    id char(26),
    username varchar(32) not null,

    PRIMARY KEY (id),
    UNIQUE (username)
)