use super::schema::samples;
use super::schema::symbols;
use super::schema::transactions;
use chrono::NaiveDate;
use std::collections::hash_map::HashMap;

extern crate csv;
extern crate serde;

#[derive(Debug, Deserialize)]
pub struct AVTop {
    #[serde(rename = "Meta Data")]
    pub meta_data: AVMetaData,
    #[serde(rename = "Time Series (Daily)")]
    pub time_series: HashMap<String, AVSample>,
}

#[derive(Debug, Deserialize)]
pub struct AVMetaData {
    #[serde(rename = "1. Information")]
    information: String,
    #[serde(rename = "2. Symbol")]
    pub symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "4. Output Size")]
    output_size: String,
    #[serde(rename = "5. Time Zone")]
    time_zone: String,
}
#[derive(Debug, Deserialize)]
pub struct AVSample {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. adjusted close")]
    adj_close: String,
    #[serde(rename = "6. volume")]
    volume: String,
    #[serde(rename = "7. dividend amount")]
    div: String,
    #[serde(rename = "8. split coefficient")]
    split_coefficient: String,
}

impl AVSample {
    pub fn to_sample(&self, symbol_id: i32, date: NaiveDate) -> NewSample {
        NewSample::from_av_sample(self, symbol_id, date)
    }
}

#[derive(Identifiable, Queryable)]
pub struct Symbol {
    pub id: i32,
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub sector: Option<i32>,
}

#[derive(Debug, Identifiable, Queryable, Associations, Deserialize, Insertable)]
#[belongs_to(Symbol)]
pub struct Sample {
    id: i32,
    symbol_id: i32,
    date: NaiveDate,
    high: i32,
    open: i32,
    low: i32,
    close: i32,
    volume: i32,
    dividend: i32,
    split_coeff: i32,
}

#[derive(Debug, Associations, Deserialize, Insertable)]
#[table_name = "samples"]
pub struct NewSample {
    symbol_id: i32,
    date: NaiveDate,
    high: i32,
    open: i32,
    low: i32,
    close: i32,
    volume: i32,
    dividend: i32,
    split_coeff: i32,
}

impl NewSample {
    fn from_av_sample(av_sample: &AVSample, symbol_id: i32, date: NaiveDate) -> Self {
        NewSample {
            symbol_id,
            high: float_fix(&av_sample.high),
            open: float_fix(&av_sample.open),
            low: float_fix(&av_sample.low),
            close: float_fix(&av_sample.adj_close),
            volume: av_sample.volume.parse().expect("Unable to parse volume!"),
            dividend: float_fix(&av_sample.div),
            split_coeff: float_fix(&av_sample.split_coefficient),
            date,
        }
    }
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "symbols"]
pub struct NewSymbol {
    pub name: String,
    pub symbol: String,
    pub exchange: String,
    //pub sector: &'a str,
}

#[derive(Debug, Associations, Insertable, Deserialize)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub symbol_id: i32,
    pub date: NaiveDate,
    pub price: i32,
    pub volume: i32,
    pub brokerage: i32,
}

#[derive(Debug, Deserialize)]
pub struct CSVTransaction {
    pub symbol: String,
    pub date: String,
    pub price: String,
    pub volume: String,
    pub brokerage: String,
}

impl CSVTransaction {
    pub fn to_transaction(&self, symbol_id: i32) -> NewTransaction {
        NewTransaction::from_csv_transaction(self, symbol_id)
    }
}

impl NewTransaction {
    pub fn from_csv_transaction(csv: &CSVTransaction, symbol_id: i32) -> Self {
        NewTransaction {
            symbol_id,
            date: NaiveDate::parse_from_str(&csv.date, "%Y-%m-%d").unwrap(),
            price: float_fix(&csv.price),
            volume: csv.volume.parse().expect("Failed to parse volume"),
            brokerage: float_fix(&csv.brokerage),
        }
    }
}

fn float_fix(string: &str) -> i32 {
    (string
        .parse::<f32>()
        .expect("Unable to parse in float_fix!")
        * 1000.0) as i32
}
