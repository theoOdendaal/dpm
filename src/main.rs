use std::str::FromStr;

use chrono::{Months, NaiveDate};

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::conventions::day_count::DayCountConventions;
use dpm::core::interest::{InterestFraction, PresentValue};
use dpm::core::sequence::Sequence;
use dpm::iso::iso3166::CountryTwoCode;
use dpm::resources::holidays;

fn main() {
    let start = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

    let country_code: String = CountryTwoCode::from_str("ZA").unwrap().into();

    let public_holidays = holidays::load_holidays(&country_code).unwrap();

    let step = Months::new(2);
    let bdc = BusinessDayConventions::default();
    let dcc = DayCountConventions::default();

    let seq_res = NaiveDate::seq(start, end, step).business_day(&bdc, &public_holidays);
    let seq_frac = dcc.year_fraction(&seq_res);

    let rate = 0.05;
    let df = 0.06;
    let nominal = 1_000_000.0;

    let interest_fraction = f64::simple_interest_fraction(&seq_frac, &rate);
    let seq_int = f64::interest_fraction_with_nominal(&nominal, &interest_fraction);

    let discount_factors = f64::simple_pv_fraction(&seq_frac, &df);
    let present_values = f64::interest_fraction_with_nominal(&seq_int, &discount_factors);

    println!("{:?}", seq_res);
    println!("{:?}", seq_frac);
    println!("{:?}", interest_fraction);
    println!("{:?}", seq_int);
    println!("{:?}", discount_factors);
    println!("{:?}", present_values);
}
