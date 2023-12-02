-- Add migration script here
CREATE TABLE user
(
    user_id  INTEGER PRIMARY KEY AUTOINCREMENT,
    email    TEXT NOT NULL,
    password TEXT NOT NULL,
    name     TEXT NOT NULL,
    lastname TEXT NOT NULL
);

CREATE TABLE room
(
    room_id             INTEGER PRIMARY KEY,
    icon_id             INT  NOT NULL,
    owner_id            INT  NOT NULL,
    room_name           TEXT NOT NULL,
    current_temperature REAL NOT NULL DEFAULT 0,
    current_humidity    REAL NOT NULL DEFAULT 0,
    current_watthour    REAL NOT NULL DEFAULT 0,

    FOREIGN KEY (owner_id) REFERENCES user (user_id)
);

CREATE TABLE room_history
(
    room_id     INT      NOT NULL,
    temperature REAL     NOT NULL,
    humidity    REAL     NOT NULL,
    watthour    REAL     NOT NULL,
    created_at  DATETIME NOT NULL,

    FOREIGN KEY (room_id) REFERENCES room (room_id)
);

CREATE TABLE schedule
(
    schedule_id          INTEGER PRIMARY KEY AUTOINCREMENT,
    owner_id             INT     NOT NULL,
    room_id              INT     NOT NULL,

    repeat_on            TEXT CHECK (
            repeat_on IN ('monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday')
        ),
    on_from_temperature  REAL,
    off_from_temperature REAL,
    repeat_once          BOOLEAN NOT NULL,
    trigger_after_ms     INTEGER,

    FOREIGN KEY(owner_id) REFERENCES user(user_id)
);
