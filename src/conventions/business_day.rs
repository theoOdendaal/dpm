/// Business day conventions.
use std::collections::HashSet;

use chrono::{Datelike, Days, NaiveDate, Weekday};

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

// Call using [NaiveDate].business_day(ModifiedFollowing).
pub trait BusinessDay<A = Self> {
    fn business_day(
        &self,
        convention: &BusinessDayConventions,
        public_holidays: &HashSet<A>,
    ) -> Self;
}

impl BusinessDay for NaiveDate {
    fn business_day(
        &self,
        convention: &BusinessDayConventions,
        public_holidays: &HashSet<NaiveDate>,
    ) -> Self {
        match convention {
            BusinessDayConventions::Actual => *self,
            BusinessDayConventions::Following => {
                FollowingBusinessDay::adjust_to_business_day(self, public_holidays)
            }
            BusinessDayConventions::Preceding => {
                PrecedingBusinessDay::adjust_to_business_day(self, public_holidays)
            }
            BusinessDayConventions::ModifiedFollowing => {
                ModifiedFollowingBusinessDay::adjust_to_business_day(self, public_holidays)
            }
            BusinessDayConventions::ModifiedPreceding => {
                ModifiedPrecedingBusinessDay::adjust_to_business_day(self, public_holidays)
            }
        }
    }
}

impl BusinessDay<NaiveDate> for Vec<NaiveDate> {
    fn business_day(
        &self,
        convention: &BusinessDayConventions,
        public_holidays: &HashSet<NaiveDate>,
    ) -> Self {
        self.iter()
            .map(|x| x.business_day(convention, public_holidays))
            .collect()
    }
}

struct FollowingBusinessDay;
struct PrecedingBusinessDay;
struct ModifiedFollowingBusinessDay;
struct ModifiedPrecedingBusinessDay;

trait BusinessDayOperations<A = Self, B = A> {
    fn is_holiday(&self, public_holidays: &HashSet<B>) -> bool;
    fn is_weekend(&self) -> bool;
    fn is_different_month(&self, other: &Self) -> bool;
}

trait ConventionOperations<A: BusinessDayOperations, B = A> {
    fn adjust_to_business_day(date: &A, public_holidays: &HashSet<B>) -> A;
}

impl BusinessDayOperations for NaiveDate {
    fn is_holiday(&self, public_holidays: &HashSet<Self>) -> bool {
        public_holidays.contains(self)
    }

    fn is_weekend(&self) -> bool {
        matches!(self.weekday(), Weekday::Sat | Weekday::Sun)
    }

    fn is_different_month(&self, other: &Self) -> bool {
        self.year() != other.year() || self.month() != other.month()
    }
}

impl ConventionOperations<NaiveDate> for FollowingBusinessDay {
    fn adjust_to_business_day(date: &NaiveDate, public_holidays: &HashSet<NaiveDate>) -> NaiveDate {
        let mut following_date = *date;
        while following_date.is_holiday(public_holidays) || following_date.is_weekend() {
            following_date = following_date + Days::new(1);
        }
        following_date
    }
}

impl ConventionOperations<NaiveDate> for PrecedingBusinessDay {
    fn adjust_to_business_day(date: &NaiveDate, public_holidays: &HashSet<NaiveDate>) -> NaiveDate {
        let mut previous_date = *date;
        while previous_date.is_holiday(public_holidays) || previous_date.is_weekend() {
            previous_date = previous_date - Days::new(1);
        }
        previous_date
    }
}

impl ConventionOperations<NaiveDate> for ModifiedFollowingBusinessDay {
    fn adjust_to_business_day(date: &NaiveDate, public_holidays: &HashSet<NaiveDate>) -> NaiveDate {
        let mut following_date =
            FollowingBusinessDay::adjust_to_business_day(date, public_holidays);

        if date.is_different_month(&following_date) {
            following_date = PrecedingBusinessDay::adjust_to_business_day(date, public_holidays);
        }

        following_date
    }
}

impl ConventionOperations<NaiveDate> for ModifiedPrecedingBusinessDay {
    fn adjust_to_business_day(date: &NaiveDate, public_holidays: &HashSet<NaiveDate>) -> NaiveDate {
        let mut previous_date = PrecedingBusinessDay::adjust_to_business_day(date, public_holidays);

        if date.is_different_month(&previous_date) {
            previous_date = FollowingBusinessDay::adjust_to_business_day(date, public_holidays);
        }

        previous_date
    }
}
