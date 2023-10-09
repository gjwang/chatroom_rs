-- This file should undo anything in `up.sql`
CREATE TABLE persons
(
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    age  INTEGER
);
