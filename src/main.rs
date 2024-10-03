// TODO, use &[T] rather than &Vec<T>.
// TODO, implement CLI functionality.

// 1.
// Distinguish between 'static' and 'market data'.
// 'Static' is contractual data, where 'market data' is curves, rates etc.
// Each of the above should be handled separately.

// 2.
// Refer to ISDA definitions for terminology/'jargon'.

// 3.
// All functions should take Vec<f64> as self.

// TODO, all new functions should be 'const'.

use dpm::interest::compounding::{
    DiscreteCompoundingFrequencies, InterestConventions, TimeValueOfMoney,
};

fn main() {
    let conv2 = InterestConventions::Simple;
    let conv1 = InterestConventions::Continuous;

    let r = 0.06;
    let n = 1.3;

    let fv_1 = conv1.fv(&n, &r);
    let pv_1 = conv1.pv(&n, &r);
    let r_2 = conv2.rate(&n, &pv_1);
    let fv_2 = conv2.fv(&n, &r_2);
    let pv_2 = conv2.pv(&n, &r_2);

    dbg!(&r);
    dbg!(&r_2);
}
