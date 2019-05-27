CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    symbol_id INTEGER NOT NULL,
    date DATE NOT NULL,
    price INTEGER NOT NULL,
    volume INTEGER NOT NULL,
    brokerage INTEGER NOT NULL,
    FOREIGN KEY(symbol_id) REFERENCES symbols(id)
);
