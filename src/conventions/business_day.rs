/// Business day convention
use chrono::{Datelike, Days, NaiveDate, Weekday};

// TODO implement unit tests.

pub enum BusinessDayConventions {
    Actual,
    Following,
    Preceding,
    ModifiedFollowing,
    ModifiedPreceding,
}

impl Default for BusinessDayConventions {
    fn default() -> Self {
        Self::ModifiedFollowing
    }
}

pub trait BusinessDay<A, B = A> {
    fn business_day(&self, value: &A, public_holidays: &B) -> A;
}

pub trait BusinessDayOperations: Sized {
    fn is_holiday(&self, public_holidays: &[Self]) -> bool;

    fn is_weekend(&self) -> bool;

    fn get_year(&self) -> i32;

    fn get_month(&self) -> u32;

    fn add_day(&self) -> Self;

    fn sub_day(&self) -> Self;

    fn is_next_month(&self, other: &Self) -> bool {
        self.get_year() != other.get_year() || self.get_month() != other.get_month()
    }
}

impl BusinessDayOperations for NaiveDate {
    fn is_holiday(&self, public_holidays: &[Self]) -> bool {
        public_holidays.contains(self)
    }

    fn is_weekend(&self) -> bool {
        matches!(self.weekday(), Weekday::Sat | Weekday::Sun)
    }

    fn get_year(&self) -> i32 {
        self.year()
    }

    fn get_month(&self) -> u32 {
        self.month()
    }

    fn add_day(&self) -> Self {
        *self + Days::new(1)
    }

    fn sub_day(&self) -> Self {
        *self - Days::new(1)
    }
}

struct FollowingBusinessDay;
struct PrecedingBusinessDay;
struct ModifiedFollowingBusinessDay;
struct ModifiedPrecedingBusinessDay;

impl<A> BusinessDay<A, Vec<A>> for FollowingBusinessDay
where
    A: BusinessDayOperations + Copy,
{
    fn business_day(&self, value: &A, public_holidays: &Vec<A>) -> A {
        let mut following_date = *value;
        while following_date.is_holiday(public_holidays) || following_date.is_weekend() {
            following_date = following_date.add_day();
        }
        following_date
    }
}

impl<A> BusinessDay<A, Vec<A>> for PrecedingBusinessDay
where
    A: BusinessDayOperations + Copy,
{
    fn business_day(&self, value: &A, public_holidays: &Vec<A>) -> A {
        let mut preceding_date = *value;
        while preceding_date.is_holiday(public_holidays) || preceding_date.is_weekend() {
            preceding_date = preceding_date.sub_day();
        }
        preceding_date
    }
}

impl<A> BusinessDay<A, Vec<A>> for ModifiedFollowingBusinessDay
where
    A: BusinessDayOperations + Copy,
{
    fn business_day(&self, value: &A, public_holidays: &Vec<A>) -> A {
        let mut following_date = FollowingBusinessDay.business_day(value, public_holidays);
        if following_date.is_next_month(value) {
            following_date = PrecedingBusinessDay.business_day(value, public_holidays);
        }
        following_date
    }
}

impl<A> BusinessDay<A, Vec<A>> for ModifiedPrecedingBusinessDay
where
    A: BusinessDayOperations + Copy,
{
    fn business_day(&self, value: &A, public_holidays: &Vec<A>) -> A {
        let mut preceding_date = PrecedingBusinessDay.business_day(value, public_holidays);
        if preceding_date.is_next_month(value) {
            preceding_date = FollowingBusinessDay.business_day(value, public_holidays);
        }
        preceding_date
    }
}

impl<A> BusinessDay<A, Vec<A>> for BusinessDayConventions
where
    A: BusinessDayOperations + Copy,
{
    fn business_day(&self, value: &A, public_holidays: &Vec<A>) -> A {
        match self {
            Self::Actual => *value,
            Self::Following => FollowingBusinessDay.business_day(value, public_holidays),
            Self::Preceding => PrecedingBusinessDay.business_day(value, public_holidays),
            Self::ModifiedFollowing => {
                ModifiedFollowingBusinessDay.business_day(value, public_holidays)
            }
            Self::ModifiedPreceding => {
                ModifiedPrecedingBusinessDay.business_day(value, public_holidays)
            }
        }
    }
}

impl<A, B> BusinessDay<Vec<A>, Vec<A>> for B
where
    A: BusinessDayOperations + Copy,
    B: BusinessDay<A, Vec<A>>,
{
    fn business_day(&self, value: &Vec<A>, public_holidays: &Vec<A>) -> Vec<A> {
        value
            .iter()
            .copied()
            .map(|a| self.business_day(&a, public_holidays))
            .collect()
    }
}
