use std::any::type_name;
use std::fmt;
use std::fs::File;

use std::str::FromStr;
use crate::account::AccountStorage;
use crate::transactions::{TRANSACTION};

mod transactions;
mod account;

//Function to make data parsing easier (casts to values and ensures proper parsing)
fn boundary<T>(val: Option<&str>) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: fmt::Debug
{
    let raw_val = val.expect("failed to deserialize csv field").trim().parse::<T>();
    raw_val.expect(format!("failed to parse value as {}", type_name::<T>()).as_str())
}




fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("expected input file");
    let mut accounts = AccountStorage::new();
                                                                                                                //any stream that implements io::Read can be passed here
    let mut rdr = csv::ReaderBuilder::new().flexible(true).has_headers(false).from_reader(File::open(input).expect("failed to read file"));
    let mut hd_passed = false;
    for res in rdr.records() {
        if hd_passed {
            let str = res.expect("failed to encode csv field");
            let trans = TRANSACTION::from(boundary::<String>(str.get(0)).as_str());
            let client = boundary::<u16>(str.get(1));
            let tx = boundary::<u32>(str.get(2));
            let amount = if str.get(3).is_some() { boundary::<f64>(str.get(3)) } else {0.0};
            accounts.handle_transaction(client, trans.create(client, tx, amount))
        } else {
            hd_passed = true;
        }

    }
    accounts.write();
}
