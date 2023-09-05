-- Add down migration script here
ALTER TABLE collections
    DROP locked;

ALTER TABLE collections
    DROP tags;

ALTER TABLE schedules
    DROP locked;

ALTER TABLE schedules
    DROP tags;