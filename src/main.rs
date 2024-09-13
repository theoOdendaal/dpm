/*
use std::str::FromStr;

use chrono::{Months, NaiveDate};

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::core::sequence::Sequence;
use dpm::iso::iso3166::CountryTwoCode;
*/
use dpm::resources::holiday_loader;

// TODO Implement a logger for the holiday_loader module.
fn main() {
    // Holiday loader.

    //let _res = holiday_loader::write_public_holidays(&Client::new(), 2022..=2024, "US");

    let mut res = holiday_loader::HolidayLoader::new();
    println!("{:?}", &res);

    println!("{:?}", holiday_loader::read_saved_holidays_population());
    let period = 2019;
    let test = &mut res.load_if_not_available("ZA", period);
    println!("{:?}", test);
    println!("{:?}", res);
    // Date range
    /*
    let country_code = CountryTwoCode::from_str("ZA");
    let holidays = holiday_loader::load_country_calendar(&country_code.unwrap().to_string())
        .unwrap_or_default();

    let start = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let step = Months::new(2);
    let bdc = BusinessDayConventions::ModifiedFollowing;

    let seq_res = NaiveDate::seq(start, end, step).business_day(&bdc, &holidays);

    println!("{:?}", seq_res);
    */
}
