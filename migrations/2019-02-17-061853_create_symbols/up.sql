-- Your SQL goes here
CREATE TABLE sectors (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);
CREATE TABLE symbols (
    id SERIAL PRIMARY KEY,
    exchange VARCHAR NOT NULL,
    symbol VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    sector INTEGER,
    UNIQUE (symbol)
)
