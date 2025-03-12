-- Your SQL goes here
create table chat (
    id char(26),
    created_at timestamp not null default current_timestamp,
    name varchar(32),

    PRIMARY KEY (id)
);

create table chat_member(
    id char(26),
    chat_id char(26) not null,
    user_id char(26) not null,

    CONSTRAINT chat_member_id_pk PRIMARY KEY (id),
    CONSTRAINT chat_member_chat_id_user_id_uq UNIQUE (chat_id, user_id),
    CONSTRAINT chat_member_chat_id_fk FOREIGN KEY (chat_id) REFERENCES chat(id),
    CONSTRAINT chat_member_user_id_fk FOREIGN KEY (user_id) REFERENCES user(id)
);

alter table message drop column user_id;

alter table message
    add member_id char(26) not null;

alter table message
    add CONSTRAINT message_chat_id_fk FOREIGN KEY (chat_id) REFERENCES chat(id);

alter table message
    add CONSTRAINT message_member_id_fk FOREIGN KEY (member_id) REFERENCES chat_member(id);