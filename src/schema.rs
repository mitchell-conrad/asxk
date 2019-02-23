table! {
    samples (id) {
        id -> Int4,
        symbol_id -> Int4,
        date -> Date,
        open -> Int4,
        high -> Int4,
        low -> Int4,
        close -> Int4,
        volume -> Int4,
        dividend -> Int4,
        split_coeff -> Int4,
    }
}

table! {
    sectors (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    symbols (id) {
        id -> Int4,
        exchange -> Varchar,
        symbol -> Varchar,
        name -> Varchar,
        sector -> Nullable<Int4>,
    }
}

joinable!(samples -> symbols (symbol_id));

allow_tables_to_appear_in_same_query!(
    samples,
    sectors,
    symbols,
);
