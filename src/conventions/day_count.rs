use chrono::NaiveDate;

/// Day count conventions.

pub enum DayCountConventions {
    Thirty360Bond,
    ThirtyE360,
    ThirtyE360ISDA,
    ThirtyEPlus360,
    Actual360,
    Actual365Fixed,
    Actual365Actual,
    NonLeap365,
}

impl Default for DayCountConventions {
    fn default() -> Self {
        Self::Actual365Fixed
    }
}

impl<A, B> DayCountConventions
where
    A: DayCount<A, B>,
{
    fn day_count(&self, d1: A, d2: A) -> B {
        match self {
            Self::Thirty360Bond => unimplemented!(),
            Self::ThirtyE360 => unimplemented!(),
            Self::ThirtyE360ISDA => unimplemented!(),
            Self::ThirtyEPlus360 => unimplemented!(),
            Self::Actual360 => Actual360::day_count(d1, d2),
            Self::Actual365Fixed => Actual365Fixed::day_count(d1, d2),
            Self::Actual365Actual => unimplemented!(),
            Self::NonLeap365 => unimplemented!(),
        }
    }
}

//  --- Traits ---

pub trait DayCount<A, B> {
    fn day_count(d1: A, d2: A) -> B;

    fn year_fraction(d1: A, d2: A) -> B;
}

impl<T, B> DayCount<Vec<T>, Vec<B>> for Vec<T>
where
    T: DayCount<T, B> + Copy,
{
    fn day_count(d1: Vec<T>, d2: Vec<T>) -> Vec<B> {
        d1.iter()
            .zip(d2.iter())
            .map(|(a, b)| T::day_count(*a, *b))
            .collect()
    }

    fn year_fraction(d1: Vec<T>, d2: Vec<T>) -> Vec<B> {
        d1.iter()
            .zip(d2.iter())
            .map(|(a, b)| T::year_fraction(*a, *b))
            .collect()
    }
}

//  --- Structs ---
//struct Thirty360Bond;
//struct ThirtyE360;
//struct ThirtyE360ISDA;
//struct ThirtyEPlus360;
struct Actual360;
struct Actual365Fixed;
//struct Actual365Actual;
//struct NonLeap365;

//  --- Trait implementations ---

impl DayCount<NaiveDate, f64> for Actual360 {
    fn day_count(d1: NaiveDate, d2: NaiveDate) -> f64 {
        (d2 - d1).num_days() as f64
    }

    fn year_fraction(d1: NaiveDate, d2: NaiveDate) -> f64 {
        Self::day_count(d1, d2) / 360.0
    }
}

impl DayCount<NaiveDate, f64> for Actual365Fixed {
    fn day_count(d1: NaiveDate, d2: NaiveDate) -> f64 {
        (d2 - d1).num_days() as f64
    }

    fn year_fraction(d1: NaiveDate, d2: NaiveDate) -> f64 {
        Self::day_count(d1, d2) / 365.0
    }
}
