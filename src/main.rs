extern crate csv;

use std::error::Error;
use std::path::Path;
use std::process;

fn example() -> Result<(), Box<Error>> {
    let data_path = Path::new("data/actual_2018.csv");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        // .from_reader(io::stdin());
        .from_path(data_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}