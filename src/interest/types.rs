//! Conversion between rate types.

//! # Definitions
//! Spot rate: (also called zero rate) is the rate of return on a non-coupon-bearing bond.
//! Swap rate:
//! Discount factor:
//! Forward rate:
//! Par rate:

//! Formulae
//! DISCOUNT VS SWAP : (S/m).Df1 + (S/m).Df2 ... + (1+ S/m)^(nx.m).Dfx = 1 (where S = Swap rate, and Df = Discount factors)
//! DISCOUNT VS SPOT: (1 + S/m)^nm.Df = 1 (where S = Spot rate, and Df = Discount factor)
//! DISCOUNT VS FORWARD: (1+r1/m)^(-x.m) * (1+F)^(-y.m) = (1+r2/m)^(-(x+y)*m)
//! SWAP VS SPOT: (S/m).(1 + rx)^-nx.m + (S/m).(1 + rx)^-nx.m ... + (1+ S/m)^(nx.m).(1 + rx)^-nx.m = 1 (where S = Swap rate, and r = Spot rates)
//! FORWARD VS SPOT: (1+r1/m)^(x.m) * (1+F)^(y.m) = (1+r2/m)^((x+y)*m)
//! FORWARD VS SWAP: ...

// Defines additional functionality to be used in conjunction with the TermStructure trait, contained in interest::term_structure.rs.

// SWAP INTO ANYTHING IS DIFFICULT.
// DISCOUNT INTO ANYTHING IS EASIER.
// Therefore, all conversions should be facilitated through discount rates.

// https://www.youtube.com/watch?v=Sq3DG1XrFsA

// Starting from swap curve - observed from market:
// First Swap, Spot and Forward Rate's are the same.

// Only 1 discount factor, irrespective of the rate type !!!!!!!!!
// Forward rate relies on no-arbitrage idea !!!

// 2y1y forward rate, can be interpreted as the forward rate that starts in 2 years,
// and is effective for a period of 1 year afterwards.
// In order to calculate the forward rate, a swap rate that is effective for 2
// years and a swap rate that is effective for 3 years is required.

// Defintiion ------------------------
// Swap rate: The fixed interest rate exchanged for a floating rate in an
// interest rate swap agreement.

// Spot rate: The current interest rate for a zero-coupon bond maturing
// at a specified future date.

// Discount factor: A multiplier used to determine the present value of
// future cash flows reflecting the time value of money.

// Par rate: The par rate is the coupon rate on a bond that makes the
// bond's price equal to its face value (par value). In other words,
// it is the interest rate that ensures the bond trades at par.
// Solve for discount rate that gives a PV of 1 ?

// Comments --------------------------
// Spot rates is the effective rate assuming a constant rate is applied over the contract term. i.e. looked at in isolation.
// Swap rate is interrelated with other rates in the curve.
// ^ Therefore, when calculating the discount factor using swap rates, you should consider previous discount factors,
// while calculating the discount factor using the spot rates, you only use a single rate.

// The purpose of module is not to convert individual rates, but rather a curve.
use super::ops::{InterestConventions, TimeValueOfMoney};
use super::term_structure::TermStructure;

//  --- Types
type Point = (f64, f64);
type Points<'a> = (&'a [f64], &'a [f64]);

// TODO create unit tests.

pub enum RateTypes {
    Swap(InterestConventions),
    Discount,
    Spot(InterestConventions),
    Forward(InterestConventions),
    Par(InterestConventions),
}

// Swap rates are typically not quoted as annualized rates, and should therefore only be apportioned for m.

impl RateTypes {
    fn convert<A>(&self, x: &A, y: &A) -> A {
        match self {
            Self::Swap(conv) => todo!(),
            Self::Discount => todo!(),
            Self::Spot(conv) => todo!(),
            Self::Forward(conv) => todo!(),
            Self::Par(conv) => todo!(),
        }
    }
}

struct Swap(InterestConventions);
struct Discount;
struct Spot(InterestConventions);
struct Forward(InterestConventions);
struct Par(InterestConventions);

// Random functions.

pub fn discount_to_forward_vec<A>(convention: &InterestConventions, curve: &A) -> Vec<f64>
where
    A: TermStructure<f64>,
{
    let x_short = curve.get_x();
    let y_short = curve.get_y();
    // TODO rather than setting nan to zero, filter for all values that is non NaN ?
    // TODO  Refactor this once iterator is implemented for term_structure trait.
    // TODO, determine when this function would return NaN?
    x_short
        .iter()
        .zip(y_short.iter())
        .zip(x_short.iter().skip(1))
        .zip(y_short.iter().skip(1))
        .map(|(((x_s, y_s), x_l), y_l)| {
            let value = convention.rate(&(*x_l - *x_s), &(*y_l / *y_s));
            if value.is_nan() {
                0.0
            } else {
                value
            }
        })
        .collect()
}

//  --- Swap rate conversions ---
// fn swap_to_discount
// fn swap_to_spot
// fn swap_to_forward
// fn swap_to_par

// This works.
pub fn swap_to_discount(
    convention: &InterestConventions,
    swap_point: &Point,
    df_points: &Points,
) -> f64 {
    (1.0 - df_points
        .0
        .iter()
        .zip(df_points.1.iter())
        .map(|(a, b)| convention.interest(a, &swap_point.1) * b)
        .sum::<f64>())
        / (convention.fv(&swap_point.0, &swap_point.1))
}

// Double check this logic.
pub fn swap_to_spot(
    convention: &InterestConventions,
    swap_point: &Point,
    spot_points: &Points,
) -> f64 {
    (1.0 - spot_points
        .0
        .iter()
        .zip(spot_points.1.iter())
        .map(|(a, b)| convention.interest(a, &swap_point.1) * convention.pv(a, b))
        .sum::<f64>())
        / (convention.fv(&swap_point.0, &swap_point.1))
}

//  --- Discount rate conversions ---
// fn discount_to_swap
// fn discount_to_spot
// fn discount_to_forward
// fn discount_to_par

// Where Point = (n, df).
fn discount_to_spot(convention: &InterestConventions, point: &Point) -> f64 {
    convention.rate(&point.0, &point.1)
}

// Where Point = (n, df).
pub fn discount_to_forward(
    convention: &InterestConventions,
    short_point: &Point,
    long_point: &Point,
) -> f64 {
    let (short_n, short_df) = short_point;
    let (long_n, long_df) = long_point;
    convention.rate(&(long_n - short_n), &(long_df / short_df))
}

//  --- Spot rate conversions ---
// fn spot_to_swap
// fn spot_to_discount
// fn spot_to_forward
// fn spot_to_par

// Where Point = (n, r).
fn spot_to_discount(convention: &InterestConventions, point: &Point) -> f64 {
    convention.pv(&point.0, &point.1)
}
// Where Point = (n, r).
pub fn spot_to_forward(
    convention: &InterestConventions,
    short_point: &Point,
    long_point: &Point,
) -> f64 {
    let (short_n, short_r) = short_point;
    let (long_n, long_r) = long_point;
    let short_pv = convention.pv(short_n, short_r);
    let long_pv = convention.pv(long_n, long_r);
    convention.rate(&(long_n - short_n), &(long_pv / short_pv))
}

//  --- Forward rate conversions ---
// fn forward_to_swap
// fn forward_to_discount
// fn forward_to_spot
// fn forward_to_par

fn forward_to_short_spot(
    convention: &InterestConventions,
    forward_point: &Point,
    long_point: &Point,
) -> f64 {
    let (forward_n, forward_r) = forward_point;
    let (long_n, long_r) = long_point;
    let pv = convention.pv(long_n, long_r) / convention.pv(forward_n, forward_r);
    let n = long_n - forward_n;
    convention.rate(&n, &pv)
}

fn forward_to_long_spot(
    convention: &InterestConventions,
    forward_point: &Point,
    short_point: &Point,
) -> f64 {
    let (forward_n, forward_r) = forward_point;
    let (short_n, short_r) = short_point;
    let pv = convention.pv(short_n, short_r) * convention.pv(forward_n, forward_r);
    let n = short_n + forward_n;
    convention.rate(&n, &pv)
}

//  --- Par rate conversions ---
// fn par_to_swap
// fn par_to_discount
// fn par_to_spot
// fn par_to_forward

//  --- Checks ---
// (To be used with solver)

// To be used with the solver module to find the 'swap_rate' that would produce 1.0.
//(S/m).Df1 + (S/m).Df2 ... + (1+ S/m)^nm.Dfn = 1
// Returns the result of the above.
// Should be 1.0;
// Double check this logic.
pub fn discount_and_swap_check(
    convention: &InterestConventions,
    swap_rate: &f64,
    points: &Points,
) -> f64 {
    let n = points.0;
    let df = points.1;

    let mut iter = n.iter().zip(df.iter()).peekable();

    let value: f64 = std::iter::from_fn(|| {
        iter.next().map(|(a, b)| {
            if iter.peek().is_none() {
                convention.fv(a, swap_rate) * b
            } else {
                convention.interest(a, swap_rate) * b
            }
        })
    })
    .sum();

    value
}

// Should be zero.
fn forward_and_spot_check(
    convention: &InterestConventions,
    short_point: &Point,
    forward_point: &Point,
    long_point: &Point,
) -> f64 {
    let mut short = convention.fv(&short_point.0, &short_point.1);
    short *= convention.fv(&forward_point.0, &forward_point.1);
    let long = convention.fv(&long_point.0, &long_point.1);

    long - short
}

//  --- Tests ---
#[cfg(test)]
mod test_interest_types {

    use super::*;
    use crate::assert_approx_eq;
    use crate::interest::ops::DiscreteCompoundingFrequencies;

    mod test_swap_to_discount {

        use super::*;

        #[test]
        fn test_swap_to_discount_simple() {
            let df = vec![0.996489, 0.991306, 0.984494, 0.975616]; //, 0.964519];
            let n = vec![0.5, 0.5, 0.5, 0.5]; //, 0.5];
            let convention = InterestConventions::Simple;
            let swap_rate = 0.07;
            let swap_point = (0.5, swap_rate);
            let df_points = (n.as_slice(), df.as_slice());

            let last_df = swap_to_discount(&convention, &swap_point, &df_points);

            let n = vec![0.5, 0.5, 0.5, 0.5, 0.5];
            let mut df = vec![0.996489, 0.991306, 0.984494, 0.975616];
            df.push(last_df);

            let value_check = discount_and_swap_check(&convention, &swap_rate, &(&n, &df));
            assert_approx_eq!(value_check, 1.0);
        }

        #[test]
        fn test_swap_to_discount_continuous() {
            let df = vec![0.996489, 0.991306, 0.984494, 0.975616]; //, 0.964519];
            let n = vec![0.5, 0.5, 0.5, 0.5]; //, 0.5];
            let convention = InterestConventions::Continuous;
            let swap_rate = 0.07;
            let swap_point = (0.5, swap_rate);
            let df_points = (n.as_slice(), df.as_slice());

            let last_df = swap_to_discount(&convention, &swap_point, &df_points);

            let n = vec![0.5, 0.5, 0.5, 0.5, 0.5];
            let mut df = vec![0.996489, 0.991306, 0.984494, 0.975616];
            df.push(last_df);

            let value_check = discount_and_swap_check(&convention, &swap_rate, &(&n, &df));
            assert_approx_eq!(value_check, 1.0);
        }

        #[test]
        fn test_swap_to_discount_discrete() {
            let df = vec![0.996489, 0.991306, 0.984494, 0.975616]; //, 0.964519];
            let n = vec![0.5, 0.5, 0.5, 0.5]; //, 0.5];
            let convention = InterestConventions::Discrete(DiscreteCompoundingFrequencies::Monthly);
            let swap_rate = 0.07;
            let swap_point = (0.5, swap_rate);
            let df_points = (n.as_slice(), df.as_slice());

            let last_df = swap_to_discount(&convention, &swap_point, &df_points);

            let n = vec![0.5, 0.5, 0.5, 0.5, 0.5];
            let mut df = vec![0.996489, 0.991306, 0.984494, 0.975616];
            df.push(last_df);

            let value_check = discount_and_swap_check(&convention, &swap_rate, &(&n, &df));
            assert_approx_eq!(value_check, 1.0);
        }
    }
}
