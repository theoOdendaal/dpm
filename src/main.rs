use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use chrono::{Datelike, NaiveDate};
/*
use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::conventions::day_count::DayCountConventions;
use dpm::core::sequence::Sequence;
*/
use dpm::iso::iso3166::CountryTwoCode;

use dpm::resources::holidays;

// TODO Implement a logger for the holiday_loader module.
fn main() {
    let start = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

    let country_code = CountryTwoCode::from_str("ZA").unwrap();
    let country_code: String = country_code.into();

    let country_calendar: HashMap<String, HashSet<NaiveDate>> =
        holidays::PublicHolidayRequestBuilder::new()
            .country_codes(&[&country_code, "US"])
            .periods(&[start.year() as u32, end.year() as u32])
            .build()
            .fetch()
            .unwrap()
            .save()
            .unwrap()
            .into();

    let hol = country_calendar.get("ZA");
    println!("{hol:?}");

    /*
    //let step = Months::new(2);
    //let bdc = BusinessDayConventions::ModifiedFollowing;
    //let dcc = DayCountConventions::Actual360;

    let holidays = country_calendar
        .get(&country_code)
        .unwrap()
        .iter()
        .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap())
        .collect();

    let seq_res = NaiveDate::seq(start, end, step).business_day(&bdc, &holidays);
    let seq_frac = dcc.year_fraction(&seq_res);

    println!("{:?}", seq_res);
    println!("{:?}", seq_frac);
    */
}
