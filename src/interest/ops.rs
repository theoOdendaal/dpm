//! Interest calculation conventions.

// TODO, implement compounding frequencies (including conversion).
// TODO implement unit tests.
// TODO add documentations.
// TODO Cleanup this module.

// FutureValue impl for type will automatically allow use of PresentValue and InterestFraction trait.

//  --- Errors ---
// TODO Imple the required logic for this Error enum, and incorporate in code.
pub enum Error {
    MismatchedLengths,
}

// TODO implement error handling, specifically for zip iterations. Ensure vec's are of similar length.

//  --- Trait definition ---

// FIXME Should the PresentValue, FutureValue and InterestFraction traits not be combined into a single trait as below?

pub trait ToNegative {
    fn to_negative(&self) -> Self;
}

pub trait FutureValue<A, B, C = A> {
    fn simple_fv_fraction(n: &A, r: &B) -> C;

    fn discrete_fv_fraction(n: &A, r: &B, m: &f64) -> C;

    fn continuous_fv_fraction(n: &A, r: &B) -> C;

    fn sub_pv(fv: &C, pv: &f64) -> C;
}

// To be used to derive discount factors.
pub trait PresentValue<A, B, C = A>: FutureValue<A, B, C>
where
    A: ToNegative,
{
    fn simple_pv_fraction(n: &A, r: &B) -> C;

    fn discrete_pv_fraction(n: &A, r: &B, m: &f64) -> C;

    fn continuous_pv_fraction(n: &A, r: &B) -> C;
}

// To be used for interest.
pub trait InterestFraction<A, B, C = A>: FutureValue<A, B, C> {
    fn simple_interest_fraction(n: &A, r: &B) -> C;

    fn discrete_interest_fraction(n: &A, r: &B, m: &f64) -> C;

    fn continuous_interest_fraction(n: &A, r: &B) -> C;

    // TODO, call this something else more descriptive.
    fn with_nominal(pv: &A, frac: &B) -> C {
        Self::simple_interest_fraction(pv, frac)
    }
}

// Infer interest rate from present value.
// Rate conversions can be performed by first calculating
// the FutureValue using the old rate, and then using that
// FutureValue in the InferRate logic.
pub trait InferRate<A, B, C = A> {
    fn infer_simple_rate(pv: &A, n: &B) -> C;

    fn infer_discrete_rate(pv: &A, n: &B, m: &f64) -> C;

    fn infer_continious_rate(pv: &A, n: &B) -> C;
}

//  --- Trait implementations: ToNegative ---

impl ToNegative for f64 {
    fn to_negative(&self) -> Self {
        -self
    }
}

impl ToNegative for Vec<f64> {
    fn to_negative(&self) -> Self {
        self.iter().map(|a| -a).collect()
    }
}

//  --- Trait implementations: FutureValue ---
impl FutureValue<f64, f64> for f64 {
    fn simple_fv_fraction(n: &f64, r: &f64) -> f64 {
        1.0 + r * n
    }

    fn discrete_fv_fraction(n: &f64, r: &f64, m: &f64) -> f64 {
        (1.0 + r / m).powf(n * m)
    }

    fn continuous_fv_fraction(n: &f64, r: &f64) -> f64 {
        std::f64::consts::E.powf(r * n)
    }

    fn sub_pv(fv: &f64, pv: &f64) -> f64 {
        fv - pv
    }
}

impl FutureValue<Vec<f64>, f64> for f64 {
    fn simple_fv_fraction(n: &Vec<f64>, r: &f64) -> Vec<f64> {
        n.iter().map(|a| f64::simple_fv_fraction(a, r)).collect()
    }

    fn discrete_fv_fraction(n: &Vec<f64>, r: &f64, m: &f64) -> Vec<f64> {
        n.iter()
            .map(|a| f64::discrete_fv_fraction(a, r, m))
            .collect()
    }

    fn continuous_fv_fraction(n: &Vec<f64>, r: &f64) -> Vec<f64> {
        n.iter()
            .map(|a| f64::continuous_fv_fraction(a, r))
            .collect()
    }

    fn sub_pv(fv: &Vec<f64>, pv: &f64) -> Vec<f64> {
        fv.iter().map(|a| a - pv).collect()
    }
}

impl FutureValue<f64, Vec<f64>, Vec<f64>> for f64 {
    fn simple_fv_fraction(n: &f64, r: &Vec<f64>) -> Vec<f64> {
        r.iter().map(|a| f64::simple_fv_fraction(n, a)).collect()
    }

    fn discrete_fv_fraction(n: &f64, r: &Vec<f64>, m: &f64) -> Vec<f64> {
        r.iter()
            .map(|a| f64::discrete_fv_fraction(n, a, m))
            .collect()
    }

    fn continuous_fv_fraction(n: &f64, r: &Vec<f64>) -> Vec<f64> {
        r.iter()
            .map(|a| f64::continuous_fv_fraction(n, a))
            .collect()
    }

    fn sub_pv(fv: &Vec<f64>, pv: &f64) -> Vec<f64> {
        fv.iter().map(|a| a - pv).collect()
    }
}

impl FutureValue<Vec<f64>, Vec<f64>> for f64 {
    fn simple_fv_fraction(n: &Vec<f64>, r: &Vec<f64>) -> Vec<f64> {
        // TODO, refactor assert_eq.
        assert_eq!(n.len(), r.len());

        n.iter()
            .zip(r.iter())
            .map(|(a, b)| f64::simple_fv_fraction(a, b))
            .collect()
    }

    fn discrete_fv_fraction(n: &Vec<f64>, r: &Vec<f64>, m: &f64) -> Vec<f64> {
        // TODO, refactor assert_eq.
        assert_eq!(n.len(), r.len());
        n.iter()
            .zip(r.iter())
            .map(|(a, b)| f64::discrete_fv_fraction(a, b, m))
            .collect()
    }

    fn continuous_fv_fraction(n: &Vec<f64>, r: &Vec<f64>) -> Vec<f64> {
        // TODO, refactor assert_eq.
        assert_eq!(n.len(), r.len());
        n.iter()
            .zip(r.iter())
            .map(|(a, b)| f64::continuous_fv_fraction(a, b))
            .collect()
    }

    fn sub_pv(fv: &Vec<f64>, pv: &f64) -> Vec<f64> {
        fv.iter().map(|a| a - pv).collect()
    }
}

//  --- Trait implementations: InferRateFromFutureValue ---
impl InferRate<f64, f64> for f64 {
    fn infer_simple_rate(pv: &f64, n: &f64) -> f64 {
        (1.0 - pv) / (pv * n)
    }

    fn infer_discrete_rate(pv: &f64, n: &f64, m: &f64) -> f64 {
        ((1.0 - pv).powf(1.0 / (n * m)) - 1.0) * m
    }

    fn infer_continious_rate(pv: &f64, n: &f64) -> f64 {
        (1.0 / n) * ((1.0 / pv).ln())
    }
}

impl InferRate<Vec<f64>, Vec<f64>> for f64 {
    fn infer_simple_rate(pv: &Vec<f64>, n: &Vec<f64>) -> Vec<f64> {
        // TODO, refactor assert_eq.
        assert_eq!(pv.len(), n.len());
        pv.iter()
            .zip(n.iter())
            .map(|(a, b)| f64::infer_simple_rate(a, b))
            .collect()
    }

    fn infer_discrete_rate(pv: &Vec<f64>, n: &Vec<f64>, m: &f64) -> Vec<f64> {
        // TODO, refactor assert_eq.
        assert_eq!(pv.len(), n.len());
        pv.iter()
            .zip(n.iter())
            .map(|(a, b)| f64::infer_discrete_rate(a, b, m))
            .collect()
    }

    fn infer_continious_rate(pv: &Vec<f64>, n: &Vec<f64>) -> Vec<f64> {
        // TODO, refactor assert_eq.
        assert_eq!(pv.len(), n.len());
        pv.iter()
            .zip(n.iter())
            .map(|(a, b)| f64::infer_continious_rate(a, b))
            .collect()
    }
}

//  --- Implementations: Blanket ---

impl<A, B, C, D> PresentValue<A, B, C> for D
where
    A: ToNegative,
    D: FutureValue<A, B, C>,
{
    fn simple_pv_fraction(n: &A, r: &B) -> C {
        D::simple_fv_fraction(&n.to_negative(), r)
    }

    fn discrete_pv_fraction(n: &A, r: &B, m: &f64) -> C {
        D::discrete_fv_fraction(&n.to_negative(), r, m)
    }

    fn continuous_pv_fraction(n: &A, r: &B) -> C {
        D::continuous_fv_fraction(&n.to_negative(), r)
    }
}

impl<A, B, C, D> InterestFraction<A, B, C> for D
where
    D: FutureValue<A, B, C>,
{
    fn simple_interest_fraction(n: &A, r: &B) -> C {
        D::sub_pv(&D::simple_fv_fraction(n, r), &1.0)
    }

    fn discrete_interest_fraction(n: &A, r: &B, m: &f64) -> C {
        D::sub_pv(&D::discrete_fv_fraction(n, r, m), &1.0)
    }

    fn continuous_interest_fraction(n: &A, r: &B) -> C {
        D::sub_pv(&D::continuous_fv_fraction(n, r), &1.0)
    }
}
