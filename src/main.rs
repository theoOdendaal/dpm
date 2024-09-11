use std::str::FromStr;

use chrono::{Months, NaiveDate};

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::core::sequence::Sequence;
use dpm::iso::iso3166::CountryTwoCode;
use dpm::resources::holiday_loader;

fn main() {
    // Holiday loader.
    //holiday_loader::write_public_holidays(2023..=2025, "US");
    //println!("{:?}", holiday_loader::load_country_calendar("ZA"));
    //println!("{:?}", holiday_loader::load_saved_holidays_population());

    holiday_loader::update_holidays();
    //println!("{:?}", holiday_loader::load_country_calendar("ZA"));

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
