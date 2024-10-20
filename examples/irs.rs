use std::collections::BTreeMap;
use std::str::FromStr;

use chrono::{Months, NaiveDate};

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::conventions::day_count::{DayCount, DayCountConventions};
use dpm::core::sequence::Sequence;
use dpm::interest::ops::{InterestConventions, TimeValueOfMoney};
use dpm::interest::term_structure::{CurveParameters, TermStructure};
use dpm::interest::types::discount_to_forward_vec;
use dpm::iso::iso3166::CountryTwoCode;
use dpm::math::interpolation::{Interpolate, InterpolationMethod};
use dpm::resources::holidays;

use dpm::table_print;

const CLIENT_VALUE: f64 = 2_101_754.992_13;

// FIXME this example has cash flows for the periods before the valuation date.

fn main() {
    // Contractual terms.
    let start = NaiveDate::from_ymd_opt(2009, 10, 15).unwrap();
    let end = NaiveDate::from_ymd_opt(2039, 9, 23).unwrap();
    let valuation_date = NaiveDate::from_ymd_opt(2022, 12, 31).unwrap();
    let step = Months::new(3);
    let spot = 0.07258;
    let nominal = 400_000_000.0;
    let spread1 = 0.0006;
    let spread2 = 0.0;
    let country = "ZA";

    // Conventions.
    let bdc = BusinessDayConventions::default();
    let dcc = DayCountConventions::default();
    let interpolation_method = InterpolationMethod::default();
    let interest_rate_convention = InterestConventions::Simple;

    // Load holidays.
    let country_code: String = CountryTwoCode::from_str(country).unwrap().into();
    let public_holidays = holidays::load_holidays(&country_code).unwrap();

    // Load spot rates
    /*
    let path = "src/resources/curves";
    let dir = format!("{}/jibar.txt", path);
    let contents = std::fs::read_to_string(dir).unwrap();
    let spot_rates: BTreeMap<&str, f64> = serde_json::from_str(&contents).unwrap();
    let spot_rates: CurveParameters<String, f64> = spot_rates.into();
    let (spot_x, spot_y) = spot_rates.unpack();
    */

    // Date sequence.
    let seq_res = NaiveDate::seq(start, end, step);
    // TODO Inception and termination date should not be adjusted.
    let seq_res: Vec<NaiveDate> = bdc.business_day(&seq_res, &public_holidays);

    // Discount and interest fractions.
    let discount_fractions = dcc.year_fraction(&valuation_date, &seq_res[1..].to_vec());
    let interest_fractions = dcc.year_fraction(&seq_res, &seq_res[1..].to_vec());

    // Interest rate curve.
    let path = "src/resources/curves";
    let dir = format!("{}/zar_disc_csa.txt", path);
    let contents = std::fs::read_to_string(dir).unwrap();
    let curve: BTreeMap<u32, f64> = serde_json::from_str(&contents).unwrap();
    let curve: CurveParameters<f64> = curve.into();
    let (x, y) = curve.unpack_with_map_x(|a| a / 365.0);

    // Discount factors.
    let discount_factors = interpolation_method.interpolate(&x, &y, &discount_fractions);

    // Interest rates.
    // TODO Convert into a CurveParameter, to allow easier conversion.
    let discount_factors_term = CurveParameters::new(&discount_fractions, &discount_factors);
    let forward_rates = discount_to_forward_vec(&interest_rate_convention, &discount_factors_term);
    let forward_rates = forward_rates[1..].to_vec();

    let mut padded_forward_rates = vec![0.0; discount_factors.len() - forward_rates.len() - 1];
    padded_forward_rates.push(spot);
    padded_forward_rates.extend(forward_rates);

    // Add spread
    let forward_rates1: Vec<f64> = padded_forward_rates
        .iter()
        .map(|a| if a != &0.0 { a + spread1 } else { *a })
        .collect();

    let forward_rates2: Vec<f64> = padded_forward_rates
        .iter()
        .map(|a| if a != &0.0 { a + spread2 } else { *a })
        .collect();

    assert_eq!(discount_factors.len(), forward_rates1.len());
    assert_eq!(discount_factors.len(), forward_rates2.len());

    // Cash flow operations.
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

    // Print formatting.
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
