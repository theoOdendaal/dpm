use std::str::FromStr;

use chrono::{Months, NaiveDate};

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::conventions::day_count::{DayCountConventions, YearFraction};
use dpm::core::sequence::Sequence;
use dpm::iso::iso3166::CountryTwoCode;

use dpm::resources::holiday_loader;

// TODO Implement a logger for the holiday_loader module.
fn main() {
    let start = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let step = Months::new(2);
    let bdc = BusinessDayConventions::ModifiedFollowing;
    let dcc = DayCountConventions::Actual360;

    let country_code = CountryTwoCode::from_str("ZA").unwrap();
    /*
        holiday_loader::write_public_holidays(
            &Client::new(),
            start.year() as u32..=end.year() as u32,
            &country_code.to_string(),
        )
        .unwrap();
    */
    let holidays = holiday_loader::read_country_calendar(&country_code.to_string()).unwrap();

    let seq_res = NaiveDate::seq(start, end, step).business_day(&bdc, &holidays);
    let seq_frac = (&seq_res).year_fraction(&dcc);

    println!("{:?}", seq_res);
    println!("{:?}", seq_frac);
}
