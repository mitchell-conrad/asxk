extern crate csv;
extern crate diesel;
extern crate structopt;
use asxk::models::{AVTop, Sample, Symbol};
use asxk::{establish_connection, populate_samples, populate_symbols};
use diesel::query_dsl::RunQueryDsl;
use diesel::BelongingToDsl;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Instant;
use structopt::StructOpt;
use asxk::establish_psql_connection;

#[derive(Debug, StructOpt)]
pub struct Opt {
    pub symbol_file: String,
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

    for s in symbol_list {}

    // Populate prices

    let mut file = File::open("a2m.json").unwrap();
    let mut string = String::new();

    file.read_to_string(&mut string).unwrap();

    let av_samples: AVTop = serde_json::from_str(&string).unwrap();

    now = Instant::now();
    count = populate_samples(&connection, &av_samples).unwrap();
    println!(
        "Populated {} samples in {}ms",
        count,
        now.elapsed().as_millis()
    );

    let lookup = symbols
        .filter(symbol.eq("A2M"))
        .first::<Symbol>(&connection)
        .expect("afdsasdf");

    now = Instant::now();
    let sample_list = Sample::belonging_to(&lookup)
        .load::<Sample>(&connection)
        .expect("asdfadsf");
    println!(
        "queried {} samples in {}ms",
        sample_list.len(),
        now.elapsed().as_millis()
    );
}
