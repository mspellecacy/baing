-- Add down migration script here
DROP TABLE IF EXISTS collections CASCADE;
DROP TABLE IF EXISTS schedules CASCADE;
DROP TYPE IF EXISTS share_type CASCADE;
