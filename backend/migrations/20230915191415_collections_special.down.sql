-- Add down migration script here
alter table collections
    drop special;
