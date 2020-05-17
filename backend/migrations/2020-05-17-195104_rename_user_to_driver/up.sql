ALTER TABLE race_entrants
    CHANGE COLUMN user_id driver_id INTEGER NOT NULL;
ALTER TABLE users
    RENAME TO drivers;
