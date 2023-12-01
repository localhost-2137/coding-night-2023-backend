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
    room_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id INTEGER NOT NULL, 
    icon_id   INT     NOT NULL,
    owner_id  INT     NOT NULL,

    room_name TEXT    NOT NULL,
    current_temperature REAL NOT NULL,
    current_humidity REAL NOT NULL,

    FOREIGN KEY (owner_id) REFERENCES user (user_id)
);

CREATE TABLE schedule
(
    schedule_id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id     INT NOT NULL,

    repeat_on   TEXT CHECK (
            repeat_on IN ('monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday')
        ),
    DELETE_ON   DATETIME,
    TRIGGER_ON  TIME
);
