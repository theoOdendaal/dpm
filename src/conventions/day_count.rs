use chrono::{Datelike, NaiveDate};

/// Day count conventions.

// TODO refactor

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
    fn discrete_year_fraction(&self, date1: &[NaiveDate], date2: &[NaiveDate]) -> Vec<f64> {
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

    pub fn year_fraction(&self, sequence: &[NaiveDate]) -> Vec<f64> {
        let first = sequence;
        let second: Vec<NaiveDate> = sequence.iter().skip(1).copied().collect();
        self.discrete_year_fraction(first, &second)
    }
}

//  --- Traits ---

pub trait DayCount<A, B, C> {
    fn day_count(date1: A, date2: B) -> C;

    fn year_fraction(date1: A, date2: B) -> C;
}

impl<T, A, B> DayCount<&[T], &[T], Vec<B>> for A
where
    A: DayCount<T, T, B>,
    T: Copy,
{
    fn day_count(date1: &[T], date2: &[T]) -> Vec<B> {
        date1
            .iter()
            .zip(date2.iter())
            .map(|(a, b)| A::day_count(*a, *b))
            .collect()
    }

    fn year_fraction(date1: &[T], date2: &[T]) -> Vec<B> {
        date1
            .iter()
            .zip(date2.iter())
            .map(|(a, b)| A::year_fraction(*a, *b))
            .collect()
    }
}

// TODO Refactor the code so this can be used to calculate the discount days.
impl<T, A, B> DayCount<&T, &[T], Vec<B>> for A
where
    A: DayCount<T, T, B>,
    T: Copy,
{
    fn day_count(date1: &T, date2: &[T]) -> Vec<B> {
        date2.iter().map(|a| A::day_count(*date1, *a)).collect()
    }

    fn year_fraction(date1: &T, date2: &[T]) -> Vec<B> {
        date2.iter().map(|a| A::year_fraction(*date1, *a)).collect()
    }
}

//  --- Structs ---

pub struct Thirty360Bond;
pub struct ThirtyE360;
//struct ThirtyE360ISDA;
//struct ThirtyEPlus360;
pub struct Actual360;
pub struct Actual365Fixed;
//struct Actual365Actual;
//struct NonLeap365;

//  --- Trait implementations ---

impl DayCount<NaiveDate, NaiveDate, f64> for Thirty360Bond {
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

impl DayCount<NaiveDate, NaiveDate, f64> for ThirtyE360 {
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

impl DayCount<NaiveDate, NaiveDate, f64> for Actual360 {
    fn day_count(date1: NaiveDate, date2: NaiveDate) -> f64 {
        (date2 - date1).num_days() as f64
    }

    fn year_fraction(date1: NaiveDate, date2: NaiveDate) -> f64 {
        Self::day_count(date1, date2) / 360.0
    }
}

impl DayCount<NaiveDate, NaiveDate, f64> for Actual365Fixed {
    fn day_count(date1: NaiveDate, date2: NaiveDate) -> f64 {
        (date2 - date1).num_days() as f64
    }

    fn year_fraction(date1: NaiveDate, date2: NaiveDate) -> f64 {
        Self::day_count(date1, date2) / 365.0
    }
}
