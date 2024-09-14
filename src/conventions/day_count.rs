// Day count conventions.

use chrono::NaiveDate;

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

impl DayCountConventions {
    pub fn day_count(&self, d1: &NaiveDate, d2: &NaiveDate) -> f64 {
        match self {
            Self::Thirty360Bond => unimplemented!(),
            Self::ThirtyE360 => unimplemented!(),
            Self::ThirtyE360ISDA => unimplemented!(),
            Self::ThirtyEPlus360 => unimplemented!(),
            Self::Actual360 => Actual360::year_fraction(d1, d2),
            Self::Actual365Fixed => Actual365Fixed::year_fraction(d1, d2),
            Self::Actual365Actual => unimplemented!(),
            Self::NonLeap365 => unimplemented!(),
        }
    }
}

//  --- Traits ---

// Call using [NaiveDate].day_count(Actual365Fixed).
pub trait YearFraction<A> {
    fn year_fraction(d1: A, d2: A) -> f64;
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

impl YearFraction<&NaiveDate> for Actual360 {
    fn year_fraction(d1: &NaiveDate, d2: &NaiveDate) -> f64 {
        (*d2 - *d1).num_days() as f64 / 360.0
    }
}

impl YearFraction<&NaiveDate> for Actual365Fixed {
    fn year_fraction(d1: &NaiveDate, d2: &NaiveDate) -> f64 {
        (*d2 - *d1).num_days() as f64 / 365.0
    }
}
