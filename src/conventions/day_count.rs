// Day count conventions.

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

// Call using [NaiveDate].day_count(Actual365Fixed).
pub trait DayCount {
    fn day_count(&self, convention: DayCountConventions) -> Vec<f64>;
}

/*
trait DayCountFraction<A> {
    fn year_fraction(first_date: &A, second_date: &A) -> f64;
}

struct Thirty360Bond;
struct ThirtyE360;
struct ThirtyE360ISDA;
struct ThirtyEPlus360;
struct Actual360;
struct Actual365Fixed;
struct Actual365Actual;
struct NonLeap365;
*/
