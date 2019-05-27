#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate csv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::QueryDsl;
use dotenv::dotenv;
use std::env;

use crate::models::CSVTransaction;
use crate::models::NewSample;
use crate::models::NewTransaction;
use chrono::NaiveDate;
use chrono::Weekday;
use models::AVTop;
use models::NewSymbol;
use models::Symbol;
use std::error::Error;
use std::path::Path;
use std::{thread, time};

pub fn establish_psql_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Failure connecting to psql db"))
}

#[derive(Debug)]
pub enum UpdateType {
    Normal,
    Full,
}

pub fn populate_symbols(connection: &PgConnection, path: &Path) -> Result<usize, Box<Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let symbol_list: Vec<NewSymbol> = reader
        .records()
        .map(|record| {
            let str_record = record.unwrap();
            NewSymbol {
                name: str_record
                    .get(0)
                    .ok_or("Couldn't get name")
                    .unwrap()
                    .to_string(),
                symbol: str_record
                    .get(1)
                    .ok_or("Couldn't get symbol")
                    .unwrap()
                    .to_string(),
                exchange: "ASX".to_string(),
            }
        })
        .collect();

    use schema::symbols::dsl::*;
    let inserted_count = diesel::insert_into(symbols)
        .values(symbol_list)
        .on_conflict_do_nothing()
        .execute(connection)
        .expect("Failed to insert symbols");
    Ok(inserted_count)
}

pub fn populate_transactions(connection: &PgConnection, path: &Path) -> Result<usize, Box<Error>> {
    // TODO: Force csv file to have a transaction id column to make dedup simple
    let mut reader = csv::Reader::from_path(path)?;

    use schema::symbols::dsl::*;
    let transaction_list: Vec<NewTransaction> = reader
        .deserialize()
        .map(|record| {
            let record: CSVTransaction = record.expect("Failed to parse csv");
            //            let name = strip_market(&record.symbol);
            let lookup = symbols
                .filter(symbol.eq(strip_market(&record.symbol)))
                .first::<Symbol>(connection)
                .expect("Failed to find symbol in database when adding transaction");
            record.to_transaction(lookup.id)
        })
        .collect();

    use schema::transactions::dsl::*;
    let inserted_count = diesel::insert_into(transactions)
        .values(transaction_list)
        .on_conflict_do_nothing()
        .execute(connection)
        .expect("Failed to insert transactions");

    Ok(inserted_count)
}

fn strip_market(market_symbol: &str) -> &str {
    market_symbol.rsplit(':').next().unwrap()
}

pub fn populate_samples(connection: &PgConnection, av_reply: &AVTop) -> Result<usize, Box<Error>> {
    let meta_data = &av_reply.meta_data;

    let av_symbol = meta_data.symbol.rsplit(':').next().unwrap();

    let symbol_name = &av_symbol.to_string();

    use schema::symbols::dsl::*;

    let lookup = symbols
        .filter(symbol.eq(symbol_name))
        .first::<Symbol>(connection)?;

    let new_samples: Vec<NewSample> = av_reply
        .time_series
        .iter()
        .map(|(date_key, sample)| {
            sample.to_sample(
                lookup.id,
                NaiveDate::parse_from_str(date_key, "%Y-%m-%d").unwrap(),
            )
        })
        .collect();

    use schema::samples::dsl::*;

    let inserted_count = diesel::insert_into(samples)
        .values(new_samples)
        .on_conflict_do_nothing()
        .execute(connection)
        .expect("asdf");
    Ok(inserted_count)
}

pub fn get_symbol_update(
    symbol_name: &str,
    exchange_name: &str,
    last_update_date: NaiveDate,
    update_type: &UpdateType,
) -> AVTop {
    dotenv().ok();
    let output_size = match update_type {
        UpdateType::Full => "full",
        UpdateType::Normal => "compact",
    };

    let key_string = env::var("AV_API_KEY").expect("Must set av api key.");
    let request = format!("https://www.alphavantage.co/query?function={function}&symbol={exchange_str}:{symbol_str}&apikey={key}&outputsize={output_size}",
                          function = "TIME_SERIES_DAILY_ADJUSTED",
                          exchange_str = exchange_name,
                          symbol_str = symbol_name,
                          key = key_string,
                          output_size = output_size
    );
    loop {
        if let Ok(mut body) = reqwest::get(&request) {
            body.json().expect("Failed to parse response json")
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}

pub fn update_all_symbols(
    connection: &PgConnection,
    update_type: &UpdateType,
) -> Result<usize, Box<Error>> {
    use schema::symbols::dsl::*;
    let symbol_list = symbols
        .load::<Symbol>(connection)
        .expect("Failed to load symbols from database");

    let mut total = 0;
    for update_symbol in symbol_list {
        let av_samples = get_symbol_update(
            &update_symbol.symbol,
            &update_symbol.exchange,
            NaiveDate::from_isoywd(1, 2, Weekday::Mon),
            &update_type,
        );
        let update_size = av_samples.time_series.len();
        total = total
            + populate_samples(&connection, &av_samples).expect(&format!(
                "Failed to update symbol {}",
                &update_symbol.symbol
            ));
    }
    Ok(total)
}

pub fn update_symbol(
    connection: &PgConnection,
    symbol_name: &str,
    update_type: &UpdateType,
) -> Result<usize, Box<Error>> {
    use schema::symbols::dsl::*;
    let symbol_q = symbols
        .filter(symbol.eq(symbol_name))
        .first::<Symbol>(connection)?;

    let update = get_symbol_update(
        &symbol_q.name,
        &symbol_q.exchange,
        NaiveDate::from_isoywd(0, 0, Weekday::Mon),
        update_type,
    );
    Ok(0)
}
