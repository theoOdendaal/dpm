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

use dpm::interest::ops::InterestConventions;
// TODO, all new functions should be 'const'.
//use dpm::core::solver::{self, SolveEquation};
//use dpm::interest::ops::{InterestConventions, TimeValueOfMoney};
use dpm::interest::types::{discount_to_forward, discount_to_swap_check, spot_to_forward};

use dpm::core::solver::NewtonRaphson;

fn main() {
    // Forward rate solver.
    let first = (90.0, 0.982262730598449);
    let second = (181.0 / 365.0, 0.962226033210754);
    let convention = InterestConventions::Simple;
    let f = discount_to_forward(&convention, &first, &second);

    println!("{:?}", f);

    // Swap rate solver.
    /*
    let df = [0.996489, 0.991306, 0.984494, 0.975616, 0.964519];
    let n = [0.5, 0.5, 0.5, 0.5, 0.5];
    let df = &df[..5];
    let n = &n[..df.len()];
    let m = 11.0;
    let close = |a: f64| discount_to_swap_check(&a, n, &m, df);
    let res_1 = f64::solve(&0.0, close);
    dbg!(res_1);
    dbg!(discount_to_swap_check(&res_1, n, &m, df));
    */

    // Solver.
    /*
    let n = 0.75;
    let comp = dpm::interest::ops::DiscreteCompoundingFrequencies::Annually;
    let m: f64 = comp.into();

    let f_closure: Box<dyn Fn(&f64) -> f64 + '_> =
        Box::new(move |a: &f64| (1.0 + a / m).powf(n * m));

    let b_closure: Box<dyn Fn(&f64) -> f64 + '_> =
        Box::new(move |a: &f64| (n * m) * (1.0 + a / m).powf((n * m) - 1.0));

    let target = 1.12;

    let res = solver::Discrete::solve(&f_closure, &b_closure, &target);
    println!("{:?}", res);

    let res_2 = InterestConventions::Discrete(comp);
    let res_2 = res_2.fv(&n, &res);
    println!("{:?}", res_2);
    */
}
