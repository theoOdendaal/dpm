use std::collections::BTreeMap;
use std::str::FromStr;

use chrono::NaiveDate;

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::conventions::day_count::{DayCount, DayCountConventions};
use dpm::core::sequence::Sequence;
use dpm::interest::ops::{InterestConventions, TimeValueOfMoney};
use dpm::interest::term_structure::TermStructure;
use dpm::interest::types::discount_to_forward_vec;
use dpm::iso::iso3166::CountryTwoCode;
use dpm::math::interpolation::{Interpolate, InterpolationMethod};
use dpm::resources::holidays;

use dpm::resources::market_data::{load_curve, load_spot};
use dpm::table_print;
use dpm::time::periods::IntervalPeriod;

const CLIENT_VALUE: f64 = 2_101_754.992_13;

// FIXME this example has cash flows for the periods before the valuation date.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Contractual terms.
    let start = NaiveDate::from_ymd_opt(2009, 10, 15).unwrap();
    let end = NaiveDate::from_ymd_opt(2039, 9, 23).unwrap();
    let valuation_date = NaiveDate::from_ymd_opt(2022, 12, 31).unwrap();
    let step = IntervalPeriod::Months(3);
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
    let spot_rates: BTreeMap<NaiveDate, f64> = load_spot("jibar")?;
    let spot_rates: TermStructure<NaiveDate, f64> = spot_rates.into();

    // Date sequence.
    let seq_res = NaiveDate::seq(start, end, step);
    // FIXME Inception and termination date should not be adjusted.
    let seq_res: Vec<NaiveDate> = bdc.business_day(&seq_res, &public_holidays);

    // Discount and interest fractions.
    let discount_fractions = dcc.year_fraction(&valuation_date, &seq_res[1..].to_vec());
    let interest_fractions = dcc.year_fraction(&seq_res, &seq_res[1..].to_vec());

    // Interest rate curve.
    let curve: BTreeMap<u32, f64> = load_curve("zar_disc_csa")?;
    let mut curve: TermStructure<f64> = curve.into();
    curve.map_x(|a| a / 365.0);
    let (x, y) = curve.unpack();

    // Discount factors.
    let discount_factors = interpolation_method.interpolate(&x, &y, &discount_fractions);

    // Interest rates.
    let discount_factors_term = TermStructure::new(&discount_fractions, &discount_factors);
    let forward_rates = discount_to_forward_vec(&interest_rate_convention, &discount_factors_term);
    let mut forward_rate_term = TermStructure::new_with_default_pad(&seq_res[1..], &forward_rates);
    forward_rate_term.update_with(spot_rates);
    let forward_rates1 = forward_rate_term.clone().shift(spread1).get_y();
    let forward_rates2 = forward_rate_term.clone().shift(spread2).get_y();

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
    table_print!(
        TERMINAL_WIDTH,
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

    Ok(())
}
