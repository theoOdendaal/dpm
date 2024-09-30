// TODO, use &[T] rather than &Vec<T>.
// TODO, implement CLI functionality.

use chrono::{Months, NaiveDate};

// 1.
// Distinguish between 'static' and 'market data'.
// 'Static' is contractual data, where 'market data' is curves, rates etc.
// Each of the above should be handled separately.

// 2.
// Refer to ISDA definitions for terminology/'jargon'.

fn main() {
    let start = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
    let step = Months::new(5);

    //let start = 1;
    //let end = 10;
    //let step = 12;

    let res = std::iter::successors(Some(end), move |&x| {
        if (start + step) < x {
            let next = x - step;
            Some(next)
        } else if x > start {
            Some(start)
        } else {
            None
        }
    });

    let test: Vec<NaiveDate> = res.collect();
    //test.reverse();

    println!("{:?}", test);
}
