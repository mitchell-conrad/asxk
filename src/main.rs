extern crate csv;
extern crate diesel;
extern crate structopt;
use asxk::get_symbol_update;
use asxk::models::{Sample, Symbol};
use asxk::{establish_psql_connection, populate_samples, populate_symbols};
use chrono::NaiveDate;
use chrono::Weekday;
use diesel::query_dsl::RunQueryDsl;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use std::path::Path;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    pub symbol_file: String,
    pub full_update: bool,
}

fn main() {
    let opt = Opt::from_args();
    let path = Path::new(&opt.symbol_file);

    println!("{:?}", path);

    let connection = establish_psql_connection();
    let mut now = Instant::now();
    let mut count = populate_symbols(&connection, path).unwrap();
    println!(
        "Populated {} symbols in {}ms",
        count,
        now.elapsed().as_millis()
    );

    use asxk::schema::symbols::dsl::*;

    let symbol_list = symbols
        .load::<Symbol>(&connection)
        .expect("Failed to load symbols from database");

    for update_symbol in symbol_list {
        println!("[[{}]]", &update_symbol.symbol);
        now = Instant::now();
        let av_samples = get_symbol_update(
            &update_symbol.symbol,
            &update_symbol.exchange,
            NaiveDate::from_isoywd(1, 2, Weekday::Mon),
            opt.full_update,
        );
        let update_size = av_samples.time_series.len();
        println!(
            "Got update with {} samples in {}ms.",
            update_size,
            now.elapsed().as_millis()
        );
        now = Instant::now();
        count = populate_samples(&connection, &av_samples).unwrap();

        println!(
            "Updated with {} samples in {}ms",
            count,
            now.elapsed().as_millis()
        );
    }

    // Populate prices

    let lookup = symbols
        .filter(symbol.eq("CBA"))
        .first::<Symbol>(&connection)
        .expect("failed to lookup CBA");

    use asxk::schema::samples::dsl::*;
    now = Instant::now();
    let sample_list = Sample::belonging_to(&lookup)
        .order(date.desc())
        .limit(5)
        .load::<Sample>(&connection)
        .expect("failed to query samples by foreign key");
    println!(
        "queried {} samples in {}ms",
        sample_list.len(),
        now.elapsed().as_millis()
    );
}
