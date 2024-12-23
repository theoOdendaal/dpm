use std::collections::BTreeMap;
use std::str::FromStr;

use chrono::NaiveDate;

use dpm::conventions::business_day::{BusinessDay, BusinessDayConventions};
use dpm::conventions::day_count::{DayCount, DayCountConventions};
use dpm::core::sequence::Sequence;
use dpm::country::CountryTwoCode;
use dpm::interest::ops::{InterestConventions, TimeValueOfMoney};
use dpm::interest::term_structure::{Term, TermStructure};
use dpm::interest::types::discount_to_forward;
use dpm::math::interpolation::{Interpolate, InterpolationMethod};
use dpm::resources::holidays;

use dpm::resources::market_data::{load_curve, load_spot};
use dpm::time::periods::IntervalPeriod;
use dpm::{box_print, table_print};

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
    let country_code: String = CountryTwoCode::from_str(country)?.into();
    let public_holidays = holidays::load_holidays(&country_code)?;

    // Load spot rates
    let spot_rates: BTreeMap<NaiveDate, f64> = load_spot("jibar")?;
    let spot_rates: Term<NaiveDate, f64> = spot_rates.into();

    // Date sequence.
    let seq_res = NaiveDate::seq(start, end, step);
    // FIXME Inception and termination date should not be adjusted.
    let seq_res: Vec<NaiveDate> = bdc.business_day(&seq_res, &public_holidays);
    let seq_term: Term<NaiveDate> = (&seq_res).into();

    // Discount and interest fractions.
    let discount_fractions = dcc.year_fraction(&valuation_date, &seq_term.y());
    let interest_fractions = dcc.year_fraction(&seq_term.x(), &seq_term.y());

    // Discount curve.
    let discount_curve: BTreeMap<u32, f64> = load_curve("zar_disc_csa_irs")?;
    let mut discount_curve: Term<f64> = discount_curve.into();
    discount_curve.map_x(|a| a / 365.0);
    let (df_x, df_y) = discount_curve.unpack();

    // Forward rate curve.
    let forward_curve: BTreeMap<u32, f64> = load_curve("zar_swap_irs")?;
    let mut forward_curve: Term<f64> = forward_curve.into();
    forward_curve.map_x(|a| a / 365.0);
    let (f_x, f_y) = forward_curve.unpack();

    // Discount and forward rate factors.
    let discount_factors = interpolation_method.interpolate(&df_x, &df_y, &discount_fractions);
    let forward_factors = interpolation_method.interpolate(&f_x, &f_y, &discount_fractions);

    // Interest rates.
    let forward_factors_term = Term::new(&discount_fractions, &forward_factors);
    let forward_rates = discount_to_forward(&interest_rate_convention, &forward_factors_term);
    let mut forward_rate_term = Term::with_padding(&seq_term.x(), &forward_rates);
    forward_rate_term = forward_rate_term.left_join(spot_rates);
    let forward_rates1 = forward_rate_term.clone().shift_y(spread1).y();
    let forward_rates2 = forward_rate_term.clone().shift_y(spread2).y();

    assert_eq!(forward_factors.len(), forward_rates1.len());
    assert_eq!(forward_factors.len(), forward_rates2.len());

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
        seq_term.y(),
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

    let absolute_diff = format!("{:.4}", net_pv - CLIENT_VALUE);
    let relative_diff = (net_pv - CLIENT_VALUE) / net_pv * 100.0;

    box_print!(
        TERMINAL_WIDTH,
        net_pv,
        CLIENT_VALUE,
        absolute_diff,
        relative_diff
    );

    Ok(())
}
