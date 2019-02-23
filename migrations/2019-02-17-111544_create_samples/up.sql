CREATE TABLE samples (
    id INTEGER PRIMARY KEY NOT NULL,
    symbol_id INTEGER NOT NULL,
    date DATE NOT NULL,
    open INTEGER NOT NULL,
    high INTEGER NOT NULL,
    low INTEGER NOT NULL,
    close INTEGER NOT NULL,
    volume INTEGER NOT NULL,
    dividend INTEGER NOT NULL,
    split_coeff INTEGER NOT NULL,
    FOREIGN KEY(symbol_id) REFERENCES symbols(id)
)
