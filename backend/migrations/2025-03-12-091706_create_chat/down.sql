-- This file should undo anything in `up.sql`
alter table message drop FOREIGN KEY message_member_id_fk;
alter table message drop FOREIGN KEY message_chat_id_fk;
alter table message drop column member_id;
alter table message add user_id char(26) not null;
drop table if exists chat_member;
drop table if exists chat;