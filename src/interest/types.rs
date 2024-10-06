//! Conversion between rate types.

//! # Definitions
//! Spot rate: (also called zero rate) is the rate of return on a non-coupon-bearing bond.
//! Swap rate:
//! Discount factor:
//! Forward rate:
//! Par rate:
// FIXME continue with documentation!

//! Formulae
//! DISCOUNT VS SWAP : (S/m).Df1 + (S/m).Df2 ... + (1+ S/m)^nm.Dfn = 1 (where S = Swap rate, and Df = Discount factors)
//! DISCOUNT VS SPOT: (1 + S/m)^nm.Df = 1 (where S = Spot rate, and Df = Discount factor)
//! DISCOUNT VS FORWARD: ...
//! SWAP VS SPOT: DISCOUNT VS SWAP but Df is replaced with (1+r/m)^nm
//! FORWARD VS SPOT: (1+r1/m)^(xm) * (1+F)^(ym) = (1+r2/m)^((x+y)*m)
//! FORWARD VS SWAP: ...

// SWAP INTO ANYTHING IS DIFFICULT.
// DISCOUNT INTO ANYTHING IS EASIER.
// Therefore, all conversions should be facilitated through discount rates.

// https://www.youtube.com/watch?v=Sq3DG1XrFsA

// Starting from swap curve - observed from market:
// First Swap, Spot and Forward Rate's are the same.

// Only 1 discount factor, irrespective of the rate type !!!!!!!!!
// Forward rate relies on no-arbitrage idea !!!

// This module assumes that an Equivalent Annual Rate (EAR) is passed.

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

// Comments --------------------------
// Spot rates is the effective rate assuming a constant rate is applied over the contract term. i.e. looked at in isolation.
// Swap rate is interrelated with other rates in the curve.
// ^ Therefore, when calculating the discount factor using swap rates, you should consider previous discount factors,
// while calculating the discount factor using the spot rates, you only use a single rate.

use super::ops::{InterestConventions, TimeValueOfMoney};

// TODO implement solver, similar to Excel to assit with conversion.

pub enum RateTypes {
    Swap(InterestConventions),
    Discount,
    Spot(InterestConventions),
    Forward(InterestConventions),
    Par,
}

struct Swap(InterestConventions);
struct Discount;
struct Spot(InterestConventions);
struct Forward(InterestConventions);
struct Par;

fn discount_to_spot(convention: &InterestConventions, n: &f64, discount_rate: &f64) -> f64 {
    convention.rate(n, discount_rate)
}

fn spot_to_discount(convention: &InterestConventions, n: &f64, spot_rate: &f64) -> f64 {
    convention.pv(n, spot_rate)
}

// Where &(x, y) = (n, r).
pub fn spot_to_forward(
    convention: &InterestConventions,
    short: &(f64, f64),
    long: &(f64, f64),
) -> f64 {
    let (short_n, short_r) = short;
    let (long_n, long_r) = long;
    let short_pv = convention.pv(short_n, short_r);
    let long_pv = convention.pv(long_n, long_r);
    convention.rate(&(long_n - short_n), &(long_pv / short_pv))
}

// Solve for the rate at each point, and then pass to spot_to_forward fn?
// Where &(x, y) = (n, df).
pub fn discount_to_forward(
    convention: &InterestConventions,
    short: &(f64, f64),
    long: &(f64, f64),
) -> f64 {
    let (short_n, short_df) = short;
    let (long_n, long_df) = long;
    convention.rate(&(long_n - short_n), &(long_df / short_df))
}

//(S/m).Df1 + (S/m).Df2 ... + (1+ S/m)^nm.Dfn = 1
// Returns the result of the above.
pub fn discount_to_swap_check(swap_rate: &f64, n: &[f64], m: &f64, df: &[f64]) -> f64 {
    assert_eq!(n.len(), df.len());

    let mut iter = n.iter().zip(df.iter()).peekable();

    let value: f64 = std::iter::from_fn(|| {
        iter.next().map(|(a, b)| {
            if iter.peek().is_none() {
                ((1.0 + swap_rate / m).powf(a * m)) * b
            } else {
                ((swap_rate / m).powf(a * m)) * b
            }
        })
    })
    .sum();

    value
}
