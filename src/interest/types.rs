//! Conversion between rate types.

//! # Definitions
//! Spot rate: (also called zero rate) is the rate of return on a non-coupon-bearing bond.
//! Swap rate:
//! Discount factor:
//! Forward rate:
// FIXME continue with documentation.

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

// FIXME update the logic within this module, as currently only semi-annual compounding work.

#[derive(Debug, PartialEq)]
pub enum RateTypes {
    Swap,
    Discount,
    Spot,
    Forward,
}
