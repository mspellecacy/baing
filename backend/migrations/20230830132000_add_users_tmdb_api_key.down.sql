-- Add down migration script here
ALTER TABLE users
    DROP tmdb_api_key;