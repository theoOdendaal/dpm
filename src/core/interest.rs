// TODO implement unit tests.

// FutureValue impl for type will automatically allow use of PresentValue and InterestFraction trait.

//  --- Errors ---

// TODO implement error handling, specifically for zip iterations. Ensure vec's are of similar length.

//  --- Trait definition ---

// To be used when compounding frequency is lessor than payment frequency, i.e. to derive new dirty nominal.
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

    fn interest_fraction_with_nominal(pv: &A, frac: &B) -> C {
        Self::simple_interest_fraction(pv, frac)
    }
}

pub trait ToNegative {
    fn to_negative(&self) -> Self;
}

// TODO implement
//pub trait CompoundingFrequencyConversion<A> {}

//  --- Trait implementations ---

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
        // TODO, refactor.
        assert_eq!(n.len(), r.len());

        n.iter()
            .zip(r.iter())
            .map(|(a, b)| f64::simple_fv_fraction(a, b))
            .collect()
    }

    fn discrete_fv_fraction(n: &Vec<f64>, r: &Vec<f64>, m: &f64) -> Vec<f64> {
        // TODO, refactor.
        assert_eq!(n.len(), r.len());
        n.iter()
            .zip(r.iter())
            .map(|(a, b)| f64::discrete_fv_fraction(a, b, m))
            .collect()
    }

    fn continuous_fv_fraction(n: &Vec<f64>, r: &Vec<f64>) -> Vec<f64> {
        // TODO, refactor.
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
