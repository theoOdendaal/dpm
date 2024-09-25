use std::collections::BTreeMap;
use std::str::FromStr;

use chrono::{Months, NaiveDate};

// TODO, use &[T] rather than &Vec<T>.
// TODO, implement CLI functionality.

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::conventions::day_count::DayCountConventions;
use dpm::core::sequence::Sequence;

use dpm::core::curves::{Curve, CurveParameters};
use dpm::interest::ops::InterestFraction;
use dpm::iso::iso3166::CountryTwoCode;
use dpm::math::interpolation::{self, Interpolate};
use dpm::resources::holidays;

fn main() {
    let x = vec!["2023-12-31", "2023-12-31"];
    //let x = vec![0.06, 0.07];
    let y = vec![0.06, 0.07];
    let curve = CurveParameters::new(&x, &y);

    //let trans_res = curve.map_x(|a| a + 1.0);
    if let Ok(trans_res) = curve.try_map_x(|a| NaiveDate::parse_from_str(a, "%Y-%m-%d")) {
        let curve = CurveParameters::new(&trans_res, &y);

        println!("{:?}", curve);
    } else {
        println!("Error")
    }

    /*
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
    let nominal = 1_000_000.0;

    let interest_fraction = f64::simple_interest_fraction(&interest_fractions, &rate);
    let seq_int = f64::with_nominal(&nominal, &interest_fraction);

    // Discounting
    let path = "src/resources/curves";
    let dir = format!("{}/zar_disc_csa.txt", path);
    let contents = std::fs::read_to_string(dir).unwrap();

    let curve: BTreeMap<u32, f64> = serde_json::from_str(&contents).unwrap();
    let mut curve: CurveParameters<f64> = curve.into();
    curve.transform_x(|a| a / 365.0);
    curve.transform_y(|a| a + 0.001);
    let (x, y) = (curve.get_x(), curve.get_y());

    let discount_factors = interpolation::Exponential::interpolate(x, y, &discount_fractions);

    let present_values = f64::with_nominal(&seq_int, &discount_factors);

    println!("{:?}", &seq_res);
    println!("{:?}", &interest_fractions);
    println!("{:?}", &discount_fractions);
    println!("{:?}", &interest_fraction);
    println!("{:?}", &seq_int);
    println!("{:?}", &discount_factors);
    println!("{:?}", &present_values);
    println!("{:?}", &present_values.iter().sum::<f64>());
    */
}
