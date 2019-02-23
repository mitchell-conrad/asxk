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
);
CREATE TABLE samples (
    id SERIAL PRIMARY KEY,
    symbol_id INTEGER NOT NULL,
    date DATE NOT NULL,
    open INTEGER NOT NULL,
    high INTEGER NOT NULL,
    low INTEGER NOT NULL,
    close INTEGER NOT NULL,
    volume INTEGER NOT NULL,
    dividend INTEGER NOT NULL,
    split_coeff INTEGER NOT NULL,
    FOREIGN KEY(symbol_id) REFERENCES symbols(id),
    UNIQUE (date, symbol_id)
);