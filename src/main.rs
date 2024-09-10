use std::str::FromStr;

use chrono::{Months, NaiveDate};

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::core::sequence::Sequence;
use dpm::iso::iso3166::CountryTwoCode;
use dpm::resources::load;

fn main() {
    let country_code = CountryTwoCode::from_str("ZA");
    let holidays = load::load_holidays(&country_code.unwrap().to_string()).unwrap_or_default();

    let start = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let step = Months::new(2);
    let bdc = BusinessDayConventions::ModifiedFollowing;

    let seq_res = NaiveDate::seq(start, end, step).business_day(&bdc, &holidays);

    println!("{:?}", seq_res);
}
