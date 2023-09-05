-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users
(
    id          UUID            NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    name        VARCHAR(100)    NOT NULL,
    email       VARCHAR(255)    NOT NULL UNIQUE,
    photo       VARCHAR         NOT NULL DEFAULT 'default.png',
    verified    BOOLEAN         NOT NULL DEFAULT FALSE,
    password    VARCHAR(100)    NOT NULL,
    role        VARCHAR(50)     NOT NULL DEFAULT 'user',
    created_at  TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX users_email_idx ON users (email);
