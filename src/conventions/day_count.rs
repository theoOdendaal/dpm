//! Day count conventions.

use chrono::{Datelike, NaiveDate};

// TODO refactor

/*
Day count conventions from Refinitiv:
30/360
30/360 US
30/360 GER
30/360 ISDA
30/365 ISDA
30/365 GER
30/365 BRA
30/Actual GER
30/Actual
30/Actual ISDA
30E/360 ISMA
Actual/360
Actual/364
Actual/365
Actual/Actual
Actual/Actual ISDA
Actual/Actual AFB
WorkingDays/252
Actual/365LP
Actual/365P
ActualLeapDay/365
ActualLeapDay/360
Actual/36525
Actual/Actual CAD Convention
*/

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

//  --- Custom Traits ---
pub trait DayCount<A, B, C> {
    fn day_count(&self, start: &A, end: &B) -> C;

    fn year_fraction(&self, start: &A, end: &B) -> C;
}

//  --- Structs ---

struct Thirty360Bond;
struct ThirtyE360;
//struct ThirtyE360ISDA;
//struct ThirtyEPlus360;
struct Actual360;
struct Actual365Fixed;
//struct Actual365Actual;
//struct NonLeap365;

//  --- Concrete trait implementations ---

impl DayCount<NaiveDate, NaiveDate, f64> for DayCountConventions {
    fn day_count(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        match self {
            Self::Thirty360Bond => Thirty360Bond.day_count(start, end),
            Self::ThirtyE360 => ThirtyE360.day_count(start, end),
            Self::ThirtyE360ISDA => unimplemented!(),
            Self::ThirtyEPlus360 => unimplemented!(),
            Self::Actual360 => Actual360.day_count(start, end),
            Self::Actual365Fixed => Actual365Fixed.day_count(start, end),
            Self::Actual365Actual => unimplemented!(),
            Self::NonLeap365 => unimplemented!(),
        }
    }

    fn year_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        let mut value = match self {
            Self::Thirty360Bond => Thirty360Bond.year_fraction(start, end),
            Self::ThirtyE360 => ThirtyE360.year_fraction(start, end),
            Self::ThirtyE360ISDA => unimplemented!(),
            Self::ThirtyEPlus360 => unimplemented!(),
            Self::Actual360 => Actual360.year_fraction(start, end),
            Self::Actual365Fixed => Actual365Fixed.year_fraction(start, end),
            Self::Actual365Actual => unimplemented!(),
            Self::NonLeap365 => unimplemented!(),
        };
        value = value.max(0.0);
        value
    }
}

impl DayCount<NaiveDate, NaiveDate, f64> for Thirty360Bond {
    fn day_count(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        let (y1, m1, d1) = (
            start.year() as f64,
            start.month() as f64,
            start.day().min(30) as f64,
        );
        let (y2, m2, mut d2) = (end.year() as f64, end.month() as f64, end.day() as f64);

        if d2 == 31.0 && d1 == 30.0 {
            d2 = 30.0
        }

        (y2 - y1) * 360.0 + (m2 - m1) * 30.0 + (d2 - d1)
    }

    fn year_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        self.day_count(start, end) / 360.0
    }
}

impl DayCount<NaiveDate, NaiveDate, f64> for ThirtyE360 {
    fn day_count(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        let (y1, m1, d1) = (
            start.year() as f64,
            start.month() as f64,
            start.day().min(30) as f64,
        );
        let (y2, m2, d2) = (
            end.year() as f64,
            end.month() as f64,
            end.day().min(30) as f64,
        );
        (y2 - y1) * 360.0 + (m2 - m1) * 30.0 + (d2 - d1)
    }

    fn year_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        self.day_count(start, end) / 360.0
    }
}

impl DayCount<NaiveDate, NaiveDate, f64> for Actual360 {
    fn day_count(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        (*end - *start).num_days() as f64
    }

    fn year_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        self.day_count(start, end) / 360.0
    }
}

impl DayCount<NaiveDate, NaiveDate, f64> for Actual365Fixed {
    fn day_count(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        (*end - *start).num_days() as f64
    }

    fn year_fraction(&self, start: &NaiveDate, end: &NaiveDate) -> f64 {
        self.day_count(start, end) / 365.0
    }
}

//  --- Blanket trait implementations ---

impl<A, B, C> DayCount<Vec<A>, Vec<A>, Vec<B>> for C
where
    C: DayCount<A, A, B>,
{
    fn day_count(&self, start: &Vec<A>, end: &Vec<A>) -> Vec<B> {
        //assert_eq!(start.len(), end.len()); // TODO remove this. Impl proper error handling.
        start
            .iter()
            .zip(end.iter())
            .map(|(a, b)| self.day_count(a, b))
            .collect()
    }

    fn year_fraction(&self, start: &Vec<A>, end: &Vec<A>) -> Vec<B> {
        //assert_eq!(start.len(), end.len()); // TODO remove this. Impl proper error handling.
        start
            .iter()
            .zip(end.iter())
            .map(|(a, b)| self.year_fraction(a, b))
            .collect()
    }
}

impl<A, B, C> DayCount<Vec<A>, A, Vec<B>> for C
where
    C: DayCount<A, A, B>,
{
    fn day_count(&self, start: &Vec<A>, end: &A) -> Vec<B> {
        start.iter().map(|a| self.day_count(a, end)).collect()
    }

    fn year_fraction(&self, start: &Vec<A>, end: &A) -> Vec<B> {
        start.iter().map(|a| self.year_fraction(a, end)).collect()
    }
}

impl<A, B, C> DayCount<A, Vec<A>, Vec<B>> for C
where
    C: DayCount<A, A, B>,
{
    fn day_count(&self, start: &A, end: &Vec<A>) -> Vec<B> {
        end.iter().map(|a| self.day_count(start, a)).collect()
    }

    fn year_fraction(&self, start: &A, end: &Vec<A>) -> Vec<B> {
        end.iter().map(|a| self.year_fraction(start, a)).collect()
    }
}
