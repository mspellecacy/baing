-- Add up migration script here
alter table collections
    add special varchar(32);
