-- Add up migration script here
ALTER TABLE collections
    ADD locked BOOL DEFAULT FALSE NOT NULL;

ALTER TABLE collections
    ADD tags JSONB;

ALTER TABLE schedules
    ADD locked BOOL DEFAULT FALSE NOT NULL;

ALTER TABLE schedules
    ADD tags JSONB;