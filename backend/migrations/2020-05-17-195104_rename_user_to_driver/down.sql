ALTER TABLE drivers
    RENAME TO users;
ALTER TABLE race_entrants
    CHANGE COLUMN driver_id user_id INTEGER NOT NULL;
