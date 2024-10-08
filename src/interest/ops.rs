//  --- Compounding frequencies ---

// TODO improve error handling.
// TODO add more unit tests.
// TODO add unit tests for vec's. Specifically for the pv tests.
// TODO try and remove all references to clone and copy.

// FIXME, double check logic.

// Converting between interest conventions:
// 1. Use the pv function of the current convention.
// 2. Use the pv calculated in step 1, as an argument in the rate fn called using the desired convention.
// ex.
// let rate = 0.06;
// let n = 1.2;
// let pv = Simple.pv(&n, &r);
// let new_rate = continuous.rate(&n, &pv);

//  --- Errors ---
pub enum Error {
    Invalidf64,
}

/// Interest calculation conventions.
pub enum InterestConventions {
    Simple,
    Discrete(DiscreteCompoundingFrequencies),
    Continuous,
}

#[derive(Copy, Clone, Debug)]
pub enum DiscreteCompoundingFrequencies {
    Weekly,
    Monthly,
    BiMonthly,
    Quarterly,
    TriAnnually,
    SemiAnnually,
    Annually,
}

// InterestConventions structs.
struct Simple;
struct Discrete(DiscreteCompoundingFrequencies);
struct Continuous;

pub trait TimeValueOfMoney<A, B = A, C = A> {
    /// Calculates the future value factor.
    fn fv(&self, n: &A, r: &B) -> C;

    /// Calculates the present value factor.
    fn pv(&self, n: &A, r: &B) -> C;

    /// Calculates the interest factor.
    fn interest(&self, n: &A, r: &B) -> C;

    /// Infers the interest rate.
    fn rate(&self, n: &A, pv: &B) -> C;
}

//  --- Standard library trait implementations ---

impl From<DiscreteCompoundingFrequencies> for f64 {
    fn from(value: DiscreteCompoundingFrequencies) -> Self {
        match value {
            DiscreteCompoundingFrequencies::Weekly => 52.0,
            DiscreteCompoundingFrequencies::Monthly => 12.0,
            DiscreteCompoundingFrequencies::BiMonthly => 6.0,
            DiscreteCompoundingFrequencies::Quarterly => 4.0,
            DiscreteCompoundingFrequencies::TriAnnually => 3.0,
            DiscreteCompoundingFrequencies::SemiAnnually => 2.0,
            DiscreteCompoundingFrequencies::Annually => 1.0,
        }
    }
}

impl TryFrom<f64> for DiscreteCompoundingFrequencies {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match value as u32 {
            52 => Ok(DiscreteCompoundingFrequencies::Weekly),
            12 => Ok(DiscreteCompoundingFrequencies::Monthly),
            6 => Ok(DiscreteCompoundingFrequencies::BiMonthly),
            4 => Ok(DiscreteCompoundingFrequencies::Quarterly),
            3 => Ok(DiscreteCompoundingFrequencies::TriAnnually),
            2 => Ok(DiscreteCompoundingFrequencies::SemiAnnually),
            1 => Ok(DiscreteCompoundingFrequencies::Annually),
            _ => Err(Error::Invalidf64), // TODO fix this leg.
        }
    }
}

//  --- Concrete trait implementations ---

impl TimeValueOfMoney<f64> for Simple {
    fn fv(&self, n: &f64, r: &f64) -> f64 {
        1.0 + r * n
    }

    fn pv(&self, n: &f64, r: &f64) -> f64 {
        1.0 / self.fv(n, r)
    }

    fn interest(&self, n: &f64, r: &f64) -> f64 {
        self.fv(n, r) - 1.0
    }

    fn rate(&self, n: &f64, pv: &f64) -> f64 {
        ((1.0 / pv) - 1.0) / n
    }
}

impl TimeValueOfMoney<f64> for Discrete {
    fn fv(&self, n: &f64, r: &f64) -> f64 {
        let m: f64 = self.0.into();
        (1.0 + r / m).powf(n * m)
    }

    fn pv(&self, n: &f64, r: &f64) -> f64 {
        self.fv(&-n, r)
    }

    fn interest(&self, n: &f64, r: &f64) -> f64 {
        self.fv(n, r) - 1.0
    }

    fn rate(&self, n: &f64, pv: &f64) -> f64 {
        let m: f64 = self.0.into();
        ((1.0 / pv).powf(1.0 / (n * m)) - 1.0) * m
    }
}

impl TimeValueOfMoney<f64> for Continuous {
    fn fv(&self, n: &f64, r: &f64) -> f64 {
        std::f64::consts::E.powf(r * n)
    }

    fn pv(&self, n: &f64, r: &f64) -> f64 {
        self.fv(&-n, r)
    }

    fn interest(&self, n: &f64, r: &f64) -> f64 {
        self.fv(n, r) - 1.0
    }

    fn rate(&self, n: &f64, pv: &f64) -> f64 {
        (1.0 / pv).ln() / n
    }
}

impl TimeValueOfMoney<f64> for InterestConventions {
    fn fv(&self, n: &f64, r: &f64) -> f64 {
        match self {
            Self::Simple => Simple.fv(n, r),
            Self::Discrete(x) => Discrete(*x).fv(n, r),
            Self::Continuous => Continuous.fv(n, r),
        }
    }

    fn pv(&self, n: &f64, r: &f64) -> f64 {
        match self {
            Self::Simple => Simple.pv(n, r),
            Self::Discrete(x) => Discrete(*x).pv(n, r),
            Self::Continuous => Continuous.pv(n, r),
        }
    }

    fn interest(&self, n: &f64, r: &f64) -> f64 {
        match self {
            Self::Simple => Simple.interest(n, r),
            Self::Discrete(x) => Discrete(*x).interest(n, r),
            Self::Continuous => Continuous.interest(n, r),
        }
    }

    fn rate(&self, n: &f64, pv: &f64) -> f64 {
        match self {
            Self::Simple => Simple.rate(n, pv),
            Self::Discrete(x) => Discrete(*x).rate(n, pv),
            Self::Continuous => Continuous.rate(n, pv),
        }
    }
}

//  --- Blanket trati implementations ---

impl<A, B> TimeValueOfMoney<Vec<A>, A> for B
where
    A: std::ops::Neg<Output = A>,
    B: TimeValueOfMoney<A>,
{
    fn fv(&self, n: &Vec<A>, r: &A) -> Vec<A> {
        n.iter().map(|a| self.fv(a, r)).collect()
    }

    fn pv(&self, n: &Vec<A>, r: &A) -> Vec<A> {
        n.iter().map(|a| self.pv(a, r)).collect()
    }

    fn interest(&self, n: &Vec<A>, r: &A) -> Vec<A> {
        n.iter().map(|a| self.interest(a, r)).collect()
    }

    fn rate(&self, n: &Vec<A>, pv: &A) -> Vec<A> {
        n.iter().map(|a| self.rate(a, pv)).collect()
    }
}

impl<A, B> TimeValueOfMoney<Vec<A>> for B
where
    A: std::ops::Neg<Output = A>,
    B: TimeValueOfMoney<A>,
{
    fn fv(&self, n: &Vec<A>, r: &Vec<A>) -> Vec<A> {
        n.iter().zip(r.iter()).map(|(a, b)| self.fv(a, b)).collect()
    }

    fn pv(&self, n: &Vec<A>, r: &Vec<A>) -> Vec<A> {
        n.iter().zip(r.iter()).map(|(a, b)| self.pv(a, b)).collect()
    }

    fn interest(&self, n: &Vec<A>, r: &Vec<A>) -> Vec<A> {
        n.iter()
            .zip(r.iter())
            .map(|(a, b)| self.interest(a, b))
            .collect()
    }

    fn rate(&self, n: &Vec<A>, pv: &Vec<A>) -> Vec<A> {
        n.iter()
            .zip(pv.iter())
            .map(|(a, b)| self.rate(a, b))
            .collect()
    }
}

// TODO complete.
// TODO check and test logic.
// TODO add detailed documentation.
// TODO add unit tests for the below.
// TODO the logic below does not hold true in all instances, i.e. simple fv should not be calculated by multiuplying the various fv's. Perhaps implement his for each rate type?

// The below impl should be used in instances where
// the reset frequency is more frequent than the payment frequency.
// Rather than treating each element separately, chain the values.
// e.x. calculate the FV treating each element pair as reinvestment terms.
// Essentially calculated effective factors taking into account
// multipl periods.

// interest fn assumes that returns are not reinvested, whereas the fv fn does assume returns are reinvested.
impl<A, B> TimeValueOfMoney<Vec<A>, Vec<A>, A> for B
where
    A: std::iter::Product<A> + std::iter::Sum<A> + Copy,
    B: TimeValueOfMoney<A> + TimeValueOfMoney<Vec<A>, Vec<A>, Vec<A>>,
{
    // Aggregate future value factor, taking into account multiple
    // periods.
    fn fv(&self, n: &Vec<A>, r: &Vec<A>) -> A {
        let value: Vec<A> = self.fv(n, r);
        value.into_iter().product()
    }

    // Effective present value factor, taking into
    // account multipl periods.
    fn pv(&self, n: &Vec<A>, r: &Vec<A>) -> A {
        let value: Vec<A> = self.pv(n, r);
        value.into_iter().product()
    }

    // Aggregate interest factor, taking into account
    // multiple periods.
    fn interest(&self, n: &Vec<A>, r: &Vec<A>) -> A {
        let value: Vec<A> = self.interest(n, r);
        value.into_iter().sum()
    }

    // 1 = (1+r/m)^(-n1*m) AND PV1
    // 2 = (1+r/m)^(-n2*m) AND PV2
    // Solves for the effective rate that would
    // produce the desired present value product.
    // (1+r/m)^(-n1*m) * (1+r/m)^(-n2*m) = PV1 * PV2
    // Not exactly certain when this would be useful.
    // FIXME, this logic is faulty.
    fn rate(&self, n: &Vec<A>, pv: &Vec<A>) -> A {
        let agg_pv: A = pv.iter().copied().product();
        let agg_n: A = n.iter().copied().sum();
        let rate: A = self.rate(&agg_n, &agg_pv);
        rate
    }
}

//  --- Tests ---
#[cfg(test)]
mod test_interest_ops {

    use super::*;
    use crate::assert_approx_eq;

    mod test_rate {

        use super::*;

        #[test]
        fn test_scalar_rate_simple() {
            let rate = 0.06;
            let n = 1.33;
            let convention = Simple;

            let present_value = convention.pv(&n, &rate);
            let inferred_rate = convention.rate(&n, &present_value);
            assert_approx_eq!(rate, inferred_rate)
        }

        #[test]
        fn test_vec_rate_simple() {
            let rate = 0.06;
            let n = vec![1.33, 1.54];
            let convention = Simple;

            let present_value = convention.pv(&n, &rate);
            let inferred_rate = convention.rate(&n, &present_value);
            for x in inferred_rate {
                assert_approx_eq!(rate, x);
            }
        }

        #[test]
        fn test_scalar_rate_discrete() {
            let rate = 0.06;
            let n = 1.33;

            let population = vec![
                DiscreteCompoundingFrequencies::Weekly,
                DiscreteCompoundingFrequencies::Monthly,
                DiscreteCompoundingFrequencies::BiMonthly,
                DiscreteCompoundingFrequencies::Quarterly,
                DiscreteCompoundingFrequencies::TriAnnually,
                DiscreteCompoundingFrequencies::SemiAnnually,
                DiscreteCompoundingFrequencies::Annually,
            ];

            for convention in population {
                let convention = Discrete(convention);
                let present_value = convention.pv(&n, &rate);
                let inferred_rate = convention.rate(&n, &present_value);
                assert_approx_eq!(rate, inferred_rate)
            }
        }

        #[test]
        fn test_vec_rate_discrete() {
            let rate = 0.06;
            let n = vec![1.33, 1.54];

            let population = vec![
                DiscreteCompoundingFrequencies::Weekly,
                DiscreteCompoundingFrequencies::Monthly,
                DiscreteCompoundingFrequencies::BiMonthly,
                DiscreteCompoundingFrequencies::Quarterly,
                DiscreteCompoundingFrequencies::TriAnnually,
                DiscreteCompoundingFrequencies::SemiAnnually,
                DiscreteCompoundingFrequencies::Annually,
            ];

            for convention in population {
                let convention = Discrete(convention);
                let present_value = convention.pv(&n, &rate);
                let inferred_rate = convention.rate(&n, &present_value);
                for x in inferred_rate {
                    assert_approx_eq!(rate, x);
                }
            }
        }

        #[test]
        fn test_scalar_rate_continuous() {
            let rate = 0.06;
            let n = 1.33;
            let convention = Continuous;

            let present_value = convention.pv(&n, &rate);
            let inferred_rate = convention.rate(&n, &present_value);
            assert_approx_eq!(rate, inferred_rate)
        }

        #[test]
        fn test_vec_rate_continuous() {
            let rate = 0.06;
            let n = vec![1.33, 1.54];
            let convention = Continuous;

            let present_value = convention.pv(&n, &rate);
            let inferred_rate = convention.rate(&n, &present_value);
            for x in inferred_rate {
                assert_approx_eq!(rate, x);
            }
        }
    }

    mod test_present_value {

        use super::*;

        #[test]
        fn test_scalar_pv_simple_to_discrete() {
            let rate = 0.06;
            let n = 1.57;
            let base_convention = Simple;

            let base_present_value = base_convention.pv(&n, &rate);

            let population = vec![
                DiscreteCompoundingFrequencies::Weekly,
                DiscreteCompoundingFrequencies::Monthly,
                DiscreteCompoundingFrequencies::BiMonthly,
                DiscreteCompoundingFrequencies::Quarterly,
                DiscreteCompoundingFrequencies::TriAnnually,
                DiscreteCompoundingFrequencies::SemiAnnually,
                DiscreteCompoundingFrequencies::Annually,
            ];

            for conv in population {
                let into_convention = Discrete(conv);
                let into_rate = into_convention.rate(&n, &base_present_value);
                let into_present_value = into_convention.pv(&n, &into_rate);

                assert_approx_eq!(base_present_value, into_present_value);
            }
        }

        #[test]
        fn test_scalar_pv_simple_to_continuous() {
            let rate = 0.06;
            let n = 1.57;
            let base_convention = Simple;

            let base_present_value = base_convention.pv(&n, &rate);

            let into_convention = Continuous;
            let into_rate = into_convention.rate(&n, &base_present_value);
            let into_present_value = into_convention.pv(&n, &into_rate);

            assert_approx_eq!(base_present_value, into_present_value);
        }

        #[test]
        fn test_scalar_pv_discrete_to_simple() {
            let rate = 0.06;
            let n = 1.57;

            let population = vec![
                DiscreteCompoundingFrequencies::Weekly,
                DiscreteCompoundingFrequencies::Monthly,
                DiscreteCompoundingFrequencies::BiMonthly,
                DiscreteCompoundingFrequencies::Quarterly,
                DiscreteCompoundingFrequencies::TriAnnually,
                DiscreteCompoundingFrequencies::SemiAnnually,
                DiscreteCompoundingFrequencies::Annually,
            ];

            for base_convention in population {
                let base_present_value = Discrete(base_convention).pv(&n, &rate);
                let into_convention = Simple;
                let into_rate = into_convention.rate(&n, &base_present_value);
                let into_present_value = into_convention.pv(&n, &into_rate);

                assert_approx_eq!(base_present_value, into_present_value);
            }
        }

        #[test]
        fn test_scalar_pv_discrete_to_continuous() {
            let rate = 0.06;
            let n = 1.57;

            let population = vec![
                DiscreteCompoundingFrequencies::Weekly,
                DiscreteCompoundingFrequencies::Monthly,
                DiscreteCompoundingFrequencies::BiMonthly,
                DiscreteCompoundingFrequencies::Quarterly,
                DiscreteCompoundingFrequencies::TriAnnually,
                DiscreteCompoundingFrequencies::SemiAnnually,
                DiscreteCompoundingFrequencies::Annually,
            ];

            for base_convention in population {
                let base_present_value = Discrete(base_convention).pv(&n, &rate);
                let into_convention = Continuous;
                let into_rate = into_convention.rate(&n, &base_present_value);
                let into_present_value = into_convention.pv(&n, &into_rate);

                assert_approx_eq!(base_present_value, into_present_value);
            }
        }
    }
}
