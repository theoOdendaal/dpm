/*
use std::{collections::BTreeMap, str::FromStr};

use chrono::{Months, NaiveDate};
use dpm::{
    conventions::{
        business_day::{BusinessDay, BusinessDayConventions},
        day_count::{DayCount, DayCountConventions},
    },
    core::sequence::Sequence,
    interest::term_structure::{CurveParameters, TermStructure},
    iso::iso3166::CountryTwoCode,
    math::interpolation::{self, Interpolate},
    resources::holidays,
};

const CLIENT_VALUE: f64 = -24_351_735.5;
*/

fn main() {
    /*
        let inception_date = NaiveDate::from_ymd_opt(2013, 8, 26).unwrap();
        let termination_date = NaiveDate::from_ymd_opt(2030, 9, 26).unwrap();
        let valuation_date = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        let reset_tenor = Months::new(3);
        let payment_tenor = Months::new(6);
        let fixed_rate = 0.09187;
        let bdc = BusinessDayConventions::default();
        let dcc = DayCountConventions::default();

        let country_code: String = CountryTwoCode::from_str("ZA").unwrap().into();
        let public_holidays = holidays::load_holidays(&country_code).unwrap();

        let nominal = vec![
            883_304_310.15,
            883_304_310.15,
            843_440_994.32,
            843_440_994.32,
            722_922_858.41,
            722_922_858.41,
            683_372_753.57,
            683_372_753.57,
            644_409_517.34,
            644_409_517.34,
            601_259_575.21,
            601_259_575.21,
            558_556_590.33,
            558_556_590.33,
            511_465_862.20,
            511_465_862.20,
            464_686_029.06,
            464_686_029.06,
            413_514_384.77,
            413_514_384.77,
            362_146_031.56,
            362_146_031.56,
            306_006_959.76,
            306_006_959.76,
            249_782_107.16,
            249_782_107.16,
            188_430_346.81,
            188_430_346.81,
            126_948_482.02,
            126_948_482.02,
            59_984_734.41,
            59_984_734.41,
        ];

        // Ops
        let seq_reset = Sequence::from_step(inception_date, termination_date, reset_tenor);
        let seq_payment = Sequence::from_step(inception_date, termination_date, payment_tenor);

        let seq_reset: Vec<NaiveDate> = bdc.business_day(&seq_reset.into(), &public_holidays);
        let seq_payment: Vec<NaiveDate> = bdc.business_day(&seq_payment.into(), &public_holidays);

        let discount_fractions = dcc.year_fraction(&valuation_date, &seq_payment[1..].to_vec());

        let interest_fractions = dcc.year_fraction(
            &seq_reset[..seq_reset.len() - 1].to_vec(),
            &seq_reset[1..].to_vec(),
        );

        let reset_fractions = dcc.year_fraction(&valuation_date, &seq_reset);

        let path = "src/resources/curves";
        let dir = format!("{}/zar_swap.txt", path);
        let contents = std::fs::read_to_string(dir).unwrap();
        let curve: BTreeMap<u32, f64> = serde_json::from_str(&contents).unwrap();
        let curve: CurveParameters<f64> = curve.into();

        let (x, y) = curve.unpack_with_map_x(|a| a / 365.0);

        let discount_factors = interpolation::Exponential.interpolate(&x, &y, &discount_fractions);
        let forward_factors = interpolation::Exponential.interpolate(&x, &y, &reset_fractions);

        println!("{:?}", reset_fractions);

    */
}
