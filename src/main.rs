/*
use std::collections::HashSet;
use chrono::{Months, NaiveDate};

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::core::sequence::Sequence;
*/

use dpm::resources::load;

fn main() {
    //load::update_holidays();
    println!("{:?}", load::load_holidays("ZA"));

    /*
    let start = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let step = Months::new(2);
    let holidays: HashSet<NaiveDate> = HashSet::new();
    let bdc = BusinessDayConventions::ModifiedFollowing;

    let seq_res = NaiveDate::seq(start, end, step).business_day(&bdc, &holidays);

    println!("{:?}", seq_res);
    */
}
