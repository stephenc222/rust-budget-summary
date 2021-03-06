extern crate csv;
extern crate regex;
#[macro_use] extern crate prettytable;

mod util;

use prettytable::{Row,Table};
use regex::Regex;
use std::error::Error;
use std::path::Path;
use std::process;
use util::*;

// returns the month change string to a 64 bit float
fn month_change_to_float(num_str: String ) -> f64 {
    let num_val: f64 = num_str.parse().unwrap();
    return num_val;
}

// strip the record, and turn (x) into -x for standardization
fn clean_month_change(month_str: String) -> String {
    let last: usize = month_str.len() - 1;
    let num_start_str = month_str.find('\t').unwrap_or(last) + 1;
    let prefix_removed: String = month_str[num_start_str..last].to_string();
    let prefix_last: usize = prefix_removed.len() - 1;
    let re = Regex::new(r"\((.*?)\)").unwrap();
    let clean_month_change: String = if re.is_match(&prefix_removed) {
        let mut clean_neg_str: String = strip_comma(prefix_removed[1..prefix_last].to_string());
        clean_neg_str.insert(0, '-');
        clean_neg_str
    }
    else {
        let clean_pos_str: String = strip_comma(prefix_removed).to_string();
        clean_pos_str
    };
    return clean_month_change;
}
fn read_budget() -> Result<(), Box<Error>> {
    let data_path: &Path = Path::new("data/actual_2018.csv");
    let mut table = Table::new();
    let header_row: Row = row![bFg->"Month", bFg->"Change from Last Month"];
    table.add_row(header_row);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(data_path)?;
    let mut curr_month: usize = 0;
    let mut year_sum: f64 = 0.0;
    let calendar: [String; 12] = [
        "January".to_string(),
        "February".to_string(),
        "March".to_string(),
        "April".to_string(),
        "May".to_string(),
        "June".to_string(),
        "July".to_string(),
        "August".to_string(),
        "September".to_string(),
        "October".to_string(),
        "November".to_string(),
        "December".to_string(),
    ];

    for result in rdr.records() {
        let record = result?;
        if record.get(0) == Some("") && record.get(1) != Some("Checking CF") && record.get(1) != Some("") {
            let month_change: f64 = month_change_to_float(clean_month_change(record.as_slice().to_string()));
            year_sum += month_change;
            table.add_row(row![calendar[curr_month].to_string(), r->month_change]);
            curr_month += 1;
        }
    }
    let summary_row: Row = row![bfw->"Total Year Change", rbFw->(year_sum * 100.0).round() / 100.0];
    table.add_row(summary_row);
    table.printstd();
    Ok(())
}

fn main() {
    if let Err(err) = read_budget() {
        println!("error: {}", err);
        process::exit(1);
    }
}
