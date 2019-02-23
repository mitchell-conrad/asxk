-- Your SQL goes here
CREATE TABLE sectors (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL
);
CREATE TABLE symbols (
    id INTEGER PRIMARY KEY NOT NULL,
    exchange VARCHAR NOT NULL,
    symbol VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    sector INTEGER,
    FOREIGN KEY(sector) REFERENCES sectors(id)
)
