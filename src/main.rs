use std::str::FromStr;

use chrono::{Months, NaiveDate};

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::conventions::day_count::DayCountConventions;
use dpm::core::interest::{InterestFraction, PresentValue};
use dpm::core::sequence::Sequence;
use dpm::iso::iso3166::CountryTwoCode;
use dpm::resources::holidays;

// TODO, use &[T] rather than &Vec<T>.
// TODO, implement CLI functionality.

fn main() {
    let start = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let valuation = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();
    let step = Months::new(2);
    let bdc = BusinessDayConventions::default();
    let dcc = DayCountConventions::default();

    let country_code: String = CountryTwoCode::from_str("ZA").unwrap().into();
    let public_holidays = holidays::load_holidays(&country_code).unwrap();

    let seq_res = NaiveDate::seq(start, end, step).business_day(&bdc, &public_holidays);
    let interest_fractions = dcc.interest_days_fractions(&seq_res);
    let discount_fractions = dcc.discount_days_fractions(&valuation, &seq_res[1..]);

    let rate = 0.05;
    let df = 0.06;
    let nominal = 1_000_000.0;

    let interest_fraction = f64::simple_interest_fraction(&interest_fractions, &rate);
    let seq_int = f64::with_nominal(&nominal, &interest_fraction);

    let discount_factors = f64::simple_pv_fraction(&discount_fractions, &df);
    let present_values = f64::with_nominal(&seq_int, &discount_factors);

    println!("{:?}", &seq_res);
    println!("{:?}", &interest_fractions);
    println!("{:?}", &discount_fractions);
    println!("{:?}", &interest_fraction);
    println!("{:?}", &seq_int);
    println!("{:?}", &discount_factors);
    println!("{:?}", &present_values);
    println!("{:?}", &present_values.iter().sum::<f64>());
}
