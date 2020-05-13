DROP TABLE IF EXISTS race_entrants;
DROP TABLE IF EXISTS races;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE races (
    id INTEGER PRIMARY KEY,
    track TEXT NOT NULL,
    laps INTEGER,
    minutes INTEGER
);

CREATE TABLE race_entrants (
    race_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    position INTEGER,
    vehicle TEXT,
    time INTEGER,
    best_lap INTEGER,
    lap INTEGER,
    reason INTEGER,
    ping INTEGER,
    fps INTEGER,
    fps_locked BOOLEAN,

    PRIMARY KEY (race_id, user_id),
    FOREIGN KEY (race_id) REFERENCES races(id)
        ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
