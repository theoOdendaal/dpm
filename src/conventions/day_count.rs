use chrono::{Datelike, NaiveDate};

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

impl DayCountConventions {
    pub fn year_fraction(&self, date1: &Vec<NaiveDate>, date2: &Vec<NaiveDate>) -> Vec<f64> {
        match self {
            Self::Thirty360Bond => Thirty360Bond::year_fraction(date1, date2),
            Self::ThirtyE360 => ThirtyE360::year_fraction(date1, date2),
            Self::ThirtyE360ISDA => unimplemented!(),
            Self::ThirtyEPlus360 => unimplemented!(),
            Self::Actual360 => Actual360::year_fraction(date1, date2),
            Self::Actual365Fixed => Actual365Fixed::year_fraction(date1, date2),
            Self::Actual365Actual => unimplemented!(),
            Self::NonLeap365 => unimplemented!(),
        }
    }
}

//  --- Traits ---

pub trait DayCount<A, B> {
    fn day_count(date1: A, date2: A) -> B;

    fn year_fraction(date1: A, date2: A) -> B;
}

impl<T, A, B> DayCount<&Vec<T>, Vec<B>> for A
where
    A: DayCount<T, B>,
    T: Copy,
{
    fn day_count(date1: &Vec<T>, date2: &Vec<T>) -> Vec<B> {
        date1
            .iter()
            .zip(date2.iter())
            .map(|(a, b)| A::day_count(*a, *b))
            .collect()
    }

    fn year_fraction(date1: &Vec<T>, date2: &Vec<T>) -> Vec<B> {
        date1
            .iter()
            .zip(date2.iter())
            .map(|(a, b)| A::year_fraction(*a, *b))
            .collect()
    }
}

//  --- Structs ---

struct Thirty360Bond;
struct ThirtyE360;
//struct ThirtyE360ISDA;
//struct ThirtyEPlus360;
pub struct Actual360;
struct Actual365Fixed;
//struct Actual365Actual;
//struct NonLeap365;

//  --- Trait implementations ---

impl DayCount<NaiveDate, f64> for Thirty360Bond {
    fn day_count(date1: NaiveDate, date2: NaiveDate) -> f64 {
        let (y1, m1, d1) = (
            date1.year() as f64,
            date1.month() as f64,
            date1.day().min(30) as f64,
        );
        let (y2, m2, mut d2) = (
            date2.year() as f64,
            date2.month() as f64,
            date2.day() as f64,
        );

        if d2 == 31.0 && d1 == 30.0 {
            d2 = 30.0
        }

        (y2 - y1) * 360.0 + (m2 - m1) * 30.0 + (d2 - d1)
    }

    fn year_fraction(date1: NaiveDate, date2: NaiveDate) -> f64 {
        Self::day_count(date1, date2) / 360.0
    }
}

impl DayCount<NaiveDate, f64> for ThirtyE360 {
    fn day_count(date1: NaiveDate, date2: NaiveDate) -> f64 {
        let (y1, m1, d1) = (
            date1.year() as f64,
            date1.month() as f64,
            date1.day().min(30) as f64,
        );
        let (y2, m2, d2) = (
            date2.year() as f64,
            date2.month() as f64,
            date2.day().min(30) as f64,
        );
        (y2 - y1) * 360.0 + (m2 - m1) * 30.0 + (d2 - d1)
    }

    fn year_fraction(date1: NaiveDate, date2: NaiveDate) -> f64 {
        Self::day_count(date1, date2) / 360.0
    }
}

impl DayCount<NaiveDate, f64> for Actual360 {
    fn day_count(date1: NaiveDate, date2: NaiveDate) -> f64 {
        (date2 - date1).num_days() as f64
    }

    fn year_fraction(date1: NaiveDate, date2: NaiveDate) -> f64 {
        Self::day_count(date1, date2) / 360.0
    }
}

impl DayCount<NaiveDate, f64> for Actual365Fixed {
    fn day_count(date1: NaiveDate, date2: NaiveDate) -> f64 {
        (date2 - date1).num_days() as f64
    }

    fn year_fraction(date1: NaiveDate, date2: NaiveDate) -> f64 {
        Self::day_count(date1, date2) / 365.0
    }
}
