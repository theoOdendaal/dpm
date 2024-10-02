//  --- Compounding frequencies ---

// TODO improve error handling.
// TODO remove all clone's.
// TODO rename this module to ops, once logic has been transferred.

// FIXME the logic when converting a simple rate into any other is faulty. Evidence through Simple to Simple conversion. Isolate to rate() fn.

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

struct Simple;
struct Discrete(DiscreteCompoundingFrequencies);
struct Continuous;

pub trait TimeValueOfMoney<A, B = A, C = A> {
    /// Expresses parameter A as a negative.
    fn to_negative(n: &A) -> A;

    /// Calculates the future value factor.
    fn fv(&self, n: &A, r: &B) -> C;

    /// Calculates the interest factor.
    fn interest(&self, n: &A, r: &B) -> C;

    /// Infers the interest rate.
    fn rate(&self, n: &A, pv: &B) -> C;

    //fn convert_to(&self, other: Self, n: &A, r: &B) -> (InterestConventions, C);

    /// Calculates the present value factor.
    fn pv(&self, n: &A, r: &B) -> C {
        self.fv(&Self::to_negative(n), r)
    }
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

//  --- Standard library implementations ---

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

//  --- Concrete implementations ---
impl TimeValueOfMoney<f64> for Simple {
    fn to_negative(n: &f64) -> f64 {
        -n
    }

    fn fv(&self, n: &f64, r: &f64) -> f64 {
        1.0 + r * n
    }

    fn interest(&self, n: &f64, r: &f64) -> f64 {
        self.fv(n, r) - 1.0
    }

    fn rate(&self, n: &f64, pv: &f64) -> f64 {
        ((1.0 / pv) - 1.0) / n
    }

    fn pv(&self, n: &f64, r: &f64) -> f64 {
        1.0 / self.fv(n, r)
    }
}

impl TimeValueOfMoney<f64> for Discrete {
    fn to_negative(n: &f64) -> f64 {
        -n
    }

    fn fv(&self, n: &f64, r: &f64) -> f64 {
        let m: f64 = self.0.into();
        (1.0 + r / m).powf(n * m)
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
    fn to_negative(n: &f64) -> f64 {
        -n
    }

    fn fv(&self, n: &f64, r: &f64) -> f64 {
        std::f64::consts::E.powf(r * n)
    }

    fn interest(&self, n: &f64, r: &f64) -> f64 {
        self.fv(n, r) - 1.0
    }

    fn rate(&self, n: &f64, pv: &f64) -> f64 {
        (1.0 / pv).ln() / n
    }
}

impl TimeValueOfMoney<f64> for InterestConventions {
    fn to_negative(n: &f64) -> f64 {
        -n
    }

    fn fv(&self, n: &f64, r: &f64) -> f64 {
        match self {
            Self::Simple => Simple.fv(n, r),
            Self::Discrete(x) => Discrete(*x).fv(n, r),
            Self::Continuous => Continuous.fv(n, r),
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

//  --- Blanket implementations ---
impl<A> TimeValueOfMoney<Vec<A>, A> for A
where
    A: TimeValueOfMoney<A, A, A> + std::ops::Neg<Output = A> + Clone,
{
    fn to_negative(n: &Vec<A>) -> Vec<A> {
        n.iter().map(|a| -a.clone()).collect()
    }

    fn fv(&self, n: &Vec<A>, r: &A) -> Vec<A> {
        n.iter().map(|a| self.fv(a, r)).collect()
    }

    fn interest(&self, n: &Vec<A>, r: &A) -> Vec<A> {
        n.iter().map(|a| self.interest(a, r)).collect()
    }

    fn rate(&self, n: &Vec<A>, pv: &A) -> Vec<A> {
        n.iter().map(|a| self.rate(a, pv)).collect()
    }
}

impl<A> TimeValueOfMoney<Vec<A>> for A
where
    A: TimeValueOfMoney<A, A, A> + std::ops::Neg<Output = A> + Clone,
{
    fn to_negative(n: &Vec<A>) -> Vec<A> {
        n.iter().map(|a| -a.clone()).collect()
    }

    fn fv(&self, n: &Vec<A>, r: &Vec<A>) -> Vec<A> {
        n.iter().zip(r.iter()).map(|(a, b)| self.fv(a, b)).collect()
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
            .map(|(a, b)| self.pv(a, b))
            .collect()
    }
}
