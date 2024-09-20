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

// TODO, improve this impl, as currently it doesn't allow (NaiveDate, &[NaiveDate]), used to calculate discount factors.
impl DayCountConventions {
    fn discrete_year_fraction(&self, start: &[NaiveDate], end: &[NaiveDate]) -> Vec<f64> {
        match self {
            Self::Thirty360Bond => Thirty360Bond::year_fraction(start, end),
            Self::ThirtyE360 => ThirtyE360::year_fraction(start, end),
            Self::ThirtyE360ISDA => unimplemented!(),
            Self::ThirtyEPlus360 => unimplemented!(),
            Self::Actual360 => Actual360::year_fraction(start, end),
            Self::Actual365Fixed => Actual365Fixed::year_fraction(start, end),
            Self::Actual365Actual => unimplemented!(),
            Self::NonLeap365 => unimplemented!(),
        }
    }

    pub fn discount_days_fractions(
        &self,
        evaluation: &NaiveDate,
        sequence: &[NaiveDate],
    ) -> Vec<f64> {
        let first = vec![*evaluation; sequence.len()];
        self.discrete_year_fraction(&first, sequence)
            .iter()
            .map(|x| x.max(0.0))
            .collect()
    }

    pub fn interest_days_fractions(&self, sequence: &[NaiveDate]) -> Vec<f64> {
        let first = sequence;
        let second: Vec<NaiveDate> = sequence.iter().skip(1).copied().collect();
        self.discrete_year_fraction(first, &second)
    }
}

//  --- Traits ---

pub trait DayCount<A, B, C> {
    fn day_count(start: A, end: B) -> C;

    fn year_fraction(start: A, end: B) -> C;
}

impl<T, A, B> DayCount<&[T], &[T], Vec<B>> for A
where
    A: DayCount<T, T, B>,
    T: Copy,
{
    fn day_count(start: &[T], end: &[T]) -> Vec<B> {
        start
            .iter()
            .zip(end.iter())
            .map(|(a, b)| A::day_count(*a, *b))
            .collect()
    }

    fn year_fraction(start: &[T], end: &[T]) -> Vec<B> {
        start
            .iter()
            .zip(end.iter())
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
    fn day_count(start: &T, end: &[T]) -> Vec<B> {
        end.iter().map(|a| A::day_count(*start, *a)).collect()
    }

    fn year_fraction(start: &T, end: &[T]) -> Vec<B> {
        end.iter().map(|a| A::year_fraction(*start, *a)).collect()
    }
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

//  --- Trait implementations ---

impl DayCount<NaiveDate, NaiveDate, f64> for Thirty360Bond {
    fn day_count(start: NaiveDate, end: NaiveDate) -> f64 {
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

    fn year_fraction(start: NaiveDate, end: NaiveDate) -> f64 {
        Self::day_count(start, end) / 360.0
    }
}

impl DayCount<NaiveDate, NaiveDate, f64> for ThirtyE360 {
    fn day_count(start: NaiveDate, end: NaiveDate) -> f64 {
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

    fn year_fraction(start: NaiveDate, end: NaiveDate) -> f64 {
        Self::day_count(start, end) / 360.0
    }
}

impl DayCount<NaiveDate, NaiveDate, f64> for Actual360 {
    fn day_count(start: NaiveDate, end: NaiveDate) -> f64 {
        (end - start).num_days() as f64
    }

    fn year_fraction(start: NaiveDate, end: NaiveDate) -> f64 {
        Self::day_count(start, end) / 360.0
    }
}

impl DayCount<NaiveDate, NaiveDate, f64> for Actual365Fixed {
    fn day_count(start: NaiveDate, end: NaiveDate) -> f64 {
        (end - start).num_days() as f64
    }

    fn year_fraction(start: NaiveDate, end: NaiveDate) -> f64 {
        Self::day_count(start, end) / 365.0
    }
}
