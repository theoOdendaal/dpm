use std::collections::BTreeMap;
use std::str::FromStr;

use chrono::{Months, NaiveDate};

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::conventions::day_count::{DayCount, DayCountConventions};
use dpm::core::sequence::Sequence;
use dpm::interest::ops::{InterestConventions, TimeValueOfMoney};
use dpm::interest::term_structure::{CurveParameters, TermStructure};
use dpm::iso::iso3166::CountryTwoCode;
use dpm::math::interpolation::{Interpolate, InterpolationMethod};
use dpm::resources::holidays;

use dpm::table_print;

const CLIENT_VALUE: f64 = 2_101_754.992_13;

// FIXME this example has cash flows for the periods before the valuation date.

fn main() {
    let start = NaiveDate::from_ymd_opt(2009, 10, 15).unwrap();
    let end = NaiveDate::from_ymd_opt(2039, 9, 23).unwrap();
    let valuation_date = NaiveDate::from_ymd_opt(2022, 12, 31).unwrap();
    let step = Months::new(3);
    let spot = 0.07258;
    let nominal = 400_000_000.0;
    let spread = 0.0006;
    let country = "ZA";

    let bdc = BusinessDayConventions::default();
    let dcc = DayCountConventions::default();
    let interpolation_method = InterpolationMethod::default();
    let interest_rate_convention = InterestConventions::Simple;

    let country_code: String = CountryTwoCode::from_str(country).unwrap().into();
    let public_holidays = holidays::load_holidays(&country_code).unwrap();

    let seq_res = NaiveDate::seq(start, end, step);
    let seq_res = bdc.business_day(&seq_res, &public_holidays);

    let discount_fractions = dcc.year_fraction(&valuation_date, &seq_res[1..].to_vec());
    let interest_fractions = dcc.year_fraction(&seq_res, &seq_res[1..].to_vec());

    let path = "src/resources/curves";
    let dir = format!("{}/zar_disc_csa.txt", path);
    let contents = std::fs::read_to_string(dir).unwrap();
    let curve: BTreeMap<u32, f64> = serde_json::from_str(&contents).unwrap();
    let curve: CurveParameters<f64> = curve.into();

    let (x, y) = curve.unpack_with_map_x(|a| a / 365.0);
    let discount_factors = interpolation_method.interpolate(&x, &y, &discount_fractions);

    let mut forward_rates: Vec<f64> = discount_factors
        .iter()
        .skip(1)
        .zip(discount_factors.iter())
        .zip(interest_fractions.iter())
        .map(|((a, b), c)| (b / a - 1.0) / c)
        .collect();
    forward_rates.insert(0, 0.0);

    assert_eq!(discount_factors.len(), forward_rates.len());

    // FIXME, this forward rate logic is messy. Clean up and make sure it will work on all instances.
    let last_zero_idx = forward_rates.iter().rposition(|&a| a == 0.0).map(|a| a + 1);

    let forward_rates1: Vec<f64> = forward_rates
        .iter()
        .enumerate()
        .map(|(i, &a)| {
            if Some(i) == last_zero_idx {
                spot + spread
            } else if a > 0.0 {
                a + spread
            } else {
                0.0
            }
        })
        .collect();

    let forward_rates2: Vec<f64> = forward_rates
        .iter()
        .enumerate()
        .map(|(i, &a)| {
            if Some(i) == last_zero_idx {
                spot
            } else if a > 0.0 {
                a
            } else {
                0.0
            }
        })
        .collect();

    let interest_rate_fractions1 =
        interest_rate_convention.interest(&interest_fractions, &forward_rates1);

    let interest_rate_fractions2 =
        interest_rate_convention.interest(&interest_fractions, &forward_rates2);

    let seq_int1 = interest_rate_convention.prod(&interest_rate_fractions1, &nominal);
    let seq_int2 = interest_rate_convention.prod(&interest_rate_fractions2, &nominal);
    let present_values1 = interest_rate_convention.prod(&seq_int1, &discount_factors);
    let present_values2 = interest_rate_convention.prod(&seq_int2, &discount_factors);

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
        seq_res[1..],
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
