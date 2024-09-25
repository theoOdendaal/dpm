use std::collections::BTreeMap;
use std::str::FromStr;

use chrono::{Months, NaiveDate};

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::conventions::day_count::DayCountConventions;
use dpm::core::sequence::Sequence;

use dpm::core::curves::{Curve, CurveParameters};
use dpm::interest::ops::InterestFraction;
use dpm::iso::iso3166::CountryTwoCode;
use dpm::math::interpolation::{self, Interpolate};
use dpm::resources::holidays;

use dpm::table_print;

const CLIENT_VALUE: f64 = 2_101_754.992_13;

fn main() {
    let start = NaiveDate::from_ymd_opt(2009, 10, 15).unwrap();
    let end = NaiveDate::from_ymd_opt(2039, 9, 23).unwrap();
    let valuation_date = NaiveDate::from_ymd_opt(2022, 12, 31).unwrap();
    let step = Months::new(3);
    let bdc = BusinessDayConventions::default();
    let dcc = DayCountConventions::default();

    let country_code: String = CountryTwoCode::from_str("ZA").unwrap().into();
    let public_holidays = holidays::load_holidays(&country_code).unwrap();

    let seq_res = NaiveDate::seq(start, end, step).business_day(&bdc, &public_holidays);

    let discount_fractions = dcc.discount_days_fractions(&valuation_date, &seq_res[1..]);

    let interest_fractions = dcc.interest_days_fractions(&seq_res);

    let spot = 0.07258;
    let nominal = 400_000_000.0;
    let spread = 0.0006;

    let path = "src/resources/curves";
    let dir = format!("{}/zar_disc_csa.txt", path);
    let contents = std::fs::read_to_string(dir).unwrap();

    let curve: BTreeMap<u32, f64> = serde_json::from_str(&contents).unwrap();
    let curve: CurveParameters<f64> = curve.into();

    let x = curve.map_x(|a| a / 365.0);
    let y = curve.get_y();

    let discount_factors = interpolation::LogLinear::interpolate(&x, &y, &discount_fractions);

    let mut forward_rates: Vec<f64> = discount_factors
        .iter()
        .skip(1)
        .zip(discount_factors.iter())
        .zip(interest_fractions.iter())
        .map(|((a, b), c)| (b / a - 1.0) / c)
        .collect();

    forward_rates.insert(0, 0.0);

    let forward_rates1: Vec<f64> = forward_rates
        .iter()
        .map(|a| if a <= &0.0 { spot } else { *a + spread })
        .collect();

    let forward_rates2: Vec<f64> = forward_rates
        .iter()
        .map(|a| if a <= &0.0 { spot } else { *a })
        .collect();

    let interest_rate_fractions1 =
        f64::simple_interest_fraction(&interest_fractions, &forward_rates1);

    let interest_rate_fractions2 =
        f64::simple_interest_fraction(&interest_fractions, &forward_rates2);

    let seq_int1 = f64::with_nominal(&nominal, &interest_rate_fractions1);
    let seq_int2 = f64::with_nominal(&nominal, &interest_rate_fractions2);

    let present_values1 = f64::with_nominal(&seq_int1, &discount_factors);
    let present_values2 = f64::with_nominal(&seq_int2, &discount_factors);

    let pv1_sum = present_values1.iter().sum::<f64>();
    let pv2_sum = present_values2.iter().sum::<f64>();
    let net_pv = pv1_sum - pv2_sum;

    const TERMINAL_WIDTH: usize = 200;

    let headings = &[
        "Date",
        "Interest fraction",
        "Discount fraction",
        "Discount factors",
        "Interest rate1",
        "Interest rate2",
        "Leg1 interest",
        "Leg2 interest",
        "Leg1 PV",
        "Leg2 PV",
    ];

    table_print!(
        TERMINAL_WIDTH,
        headings,
        seq_res,
        interest_fractions,
        discount_fractions,
        discount_factors,
        forward_rates1,
        forward_rates2,
        seq_int1,
        seq_int2,
        present_values1,
        present_values2
    );

    println!("{}", "-".repeat(49));
    println!("My value:\t\t\t | {:.3}\t|", &net_pv);
    println!("Client value:\t\t\t | {:.3}\t|", &CLIENT_VALUE);
    println!("Absolute difference:\t\t | {:.4}\t|", net_pv - CLIENT_VALUE);
    println!(
        "Relative difference(%):\t\t | {:.4} %\t|",
        (net_pv - CLIENT_VALUE) / net_pv * 100.0
    );
    println!("{}", "-".repeat(49));
}