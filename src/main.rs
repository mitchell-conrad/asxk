extern crate csv;
extern crate diesel;
extern crate structopt;
use asxk::update_all_symbols;
use asxk::UpdateType;
use asxk::{establish_psql_connection, populate_symbols, populate_transactions};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Opt {
    #[structopt(name = "samples")]
    Samples { full_update: bool },
    #[structopt(name = "symbols")]
    Symbols { file: String },
    #[structopt(name = "transactions")]
    Transactions {
        #[structopt(short = "f")]
        file: String,
    },
}

fn main() {
    let opt = Opt::from_args();

    let connection = establish_psql_connection();

    match opt {
        Opt::Samples { full_update } => {
            let update_type = if full_update {
                UpdateType::Full
            } else {
                UpdateType::Normal
            };
            println!("Running sample update...");
            let count =
                update_all_symbols(&connection, &update_type).expect("Failed to update samples");
            println!("Updated/inserted {} samples.", count);
        }
        Opt::Symbols { file } => {
            println!("Running symbol list update...");
            let path = Path::new(&file);
            let count = populate_symbols(&connection, path).expect("Failed to update symbols");
            println!("Updated/inserted {} symbols.", count);
        }
        Opt::Transactions { file } => {
            println!("Running transactions update...");
            let transactions_path = Path::new(&file);
            let count = populate_transactions(&connection, transactions_path)
                .expect("Failed to update transactions");
            println!("Updated/inserted {} transactions.", count);
        }
    }
}
