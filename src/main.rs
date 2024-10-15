// 1.
// Distinguish between 'static' and 'market data'.
// 'Static' is contractual data, where 'market data' is curves, rates etc.
// Each of the above should be handled separately.

// 2.
// Refer to ISDA definitions for terminology/'jargon'.

// 3.
// All functions should take Vec<f64> as self.

use chrono::{Months, NaiveDate};
use dpm::core::sequence::Sequence;

fn main() {
    let lower = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let upper = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let step = Months::new(2);

    let seq = NaiveDate::seq(lower, upper, step);

    println!("{:?}", seq);
}
