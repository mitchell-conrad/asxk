table! {
    samples (id) {
        id -> Integer,
        symbol_id -> Integer,
        date -> Date,
        open -> Integer,
        high -> Integer,
        low -> Integer,
        close -> Integer,
        volume -> Integer,
        dividend -> Integer,
        split_coeff -> Integer,
    }
}

table! {
    sectors (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    symbols (id) {
        id -> Integer,
        exchange -> Text,
        symbol -> Text,
        name -> Text,
        sector -> Nullable<Integer>,
    }
}

joinable!(samples -> symbols (symbol_id));
joinable!(symbols -> sectors (sector));

allow_tables_to_appear_in_same_query!(samples, sectors, symbols,);
