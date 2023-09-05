-- Add up migration script here
ALTER TABLE users
    ADD tmdb_api_key VARCHAR(64);