#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;
extern crate csv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::NewSample;
use chrono::NaiveDate;
use models::AVTop;
use models::NewSymbol;
use models::Symbol;
use std::error::Error;
use std::path::Path;

pub fn establish_psql_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Failure connecting to psql db"))
}

pub fn populate_symbols(connection: &PgConnection, path: &Path) -> Result<usize, Box<Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut symbol_list: Vec<NewSymbol> = Vec::new();

    symbol_list = reader
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
        .values(symbol_list).on_conflict_do_nothing()
        .execute(connection)
        .expect("Failed to insert symbols");
    Ok(inserted_count)
}

pub fn populate_samples(
    connection: &PgConnection,
    av_reply: &AVTop,
) -> Result<usize, Box<Error>> {
    let meta_data = &av_reply.meta_data;

    let av_exchange = meta_data.symbol.split(':').next().unwrap();
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
        .values(new_samples).on_conflict_do_nothing()
        .execute(connection)
        .expect("asdf");
    return Ok(inserted_count);
}
