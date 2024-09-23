use std::collections::BTreeMap;

// TODO, use &[T] rather than &Vec<T>.
// TODO, implement CLI functionality.

use dpm::core::sequence::Sequence;

use dpm::core::curves::Curve;
use dpm::interpolation::{self, Interpolate};

fn main() {
    let path = "src/resources/curves";
    let dir = format!("{}/zar_disc_csa.txt", path);
    let contents = std::fs::read_to_string(dir).unwrap();
    let curve: BTreeMap<u32, f64> = serde_json::from_str(&contents).unwrap();
    let curve = Curve::from(curve);

    let (x, y): (Vec<f64>, Vec<f64>) = curve.into();
    let xp: Vec<f64> = u32::seq(1, 1000, 1).into_iter().map(|a| a as f64).collect();
    let res = interpolation::Linear::interpolate(&x, &y, &xp);

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", res);

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
    */
}
