#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, TimeDelta};

#[derive(Serialize, Deserialize)]
struct Result {
    is_adult: bool
}

pub fn main() {
    // Read the input from the host
    let dob_and_today = sp1_zkvm::io::read::<String>();
    let parts: Vec<&str> = dob_and_today.split("|").collect();

    if parts.len() != 2 {
        panic!("Invalid dob and today input format");
    }

    let dob = parts[0];
    let today = parts[1];

    let dob_date = NaiveDate::parse_from_str(dob, "%Y-%m-%d").expect("Invalid date format");
    let today_date = NaiveDate::parse_from_str(today, "%Y-%m-%d").expect("Invalid date format");
    let age_in_days = today_date.signed_duration_since(dob_date);

    // 18 Years = 6570 days
    let is_adult = age_in_days >= TimeDelta::days(6570);

    let result = Result {
        is_adult
    };

    // Write the result to the proof
    sp1_zkvm::io::write(&result);
}
