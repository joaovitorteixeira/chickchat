create table message (
    id char(26),
    content text not null,
    chat_id char(26) not null,
    user_id char(26) not null,

    PRIMARY KEY (id, chat_id)
);

