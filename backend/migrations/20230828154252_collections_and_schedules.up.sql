
CREATE TYPE SHARE_TYPE AS ENUM ('public', 'private', 'friends');

CREATE TABLE IF NOT EXISTS collections
(
    id          UUID                PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    owner_id    UUID                NOT NULL,
    name        VARCHAR(256)        NOT NULL,
    created_at  TIMESTAMPTZ         NOT NULL DEFAULT NOW(),
    active      BOOLEAN             NOT NULL DEFAULT FALSE,
    sharing     SHARE_TYPE          NOT NULL DEFAULT 'private',
    collection  JSONB,
    CONSTRAINT FK_user_collection FOREIGN KEY(owner_id)
        REFERENCES users(id)
);
CREATE TABLE IF NOT EXISTS schedules
(
    id          UUID                PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    owner_id    UUID                NOT NULL,
    name        VARCHAR(256)        NOT NULL,
    created_at  TIMESTAMPTZ         NOT NULL DEFAULT NOW(),
    active      BOOLEAN             NOT NULL DEFAULT FALSE,
    sharing     SHARE_TYPE          NOT NULL DEFAULT 'private',
    schedule    JSONB,
    CONSTRAINT FK_user_schedule FOREIGN KEY(owner_id)
        REFERENCES users(id)
);