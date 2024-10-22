use chrono::NaiveDate;

//  --- Enums

#[derive(Clone, Copy, Debug)]
pub enum IntervalPeriod {
    Days(u32),
    Weeks(u32),
    Months(u32),
    Years(u32),
}

//  --- Trait implementations: Concrete

impl std::ops::Add<IntervalPeriod> for NaiveDate {
    type Output = NaiveDate;

    fn add(self, rhs: IntervalPeriod) -> Self::Output {
        match rhs {
            IntervalPeriod::Days(num) => self + chrono::Days::new(num as u64),
            IntervalPeriod::Weeks(num) => self + chrono::Days::new(num as u64 * 7),
            IntervalPeriod::Months(num) => self + chrono::Months::new(num),
            IntervalPeriod::Years(num) => self + chrono::Months::new(num * 12),
        }
    }
}

impl std::ops::Sub<IntervalPeriod> for NaiveDate {
    type Output = NaiveDate;

    fn sub(self, rhs: IntervalPeriod) -> Self::Output {
        match rhs {
            IntervalPeriod::Days(num) => self - chrono::Days::new(num as u64),
            IntervalPeriod::Weeks(num) => self - chrono::Days::new(num as u64 * 7),
            IntervalPeriod::Months(num) => self - chrono::Months::new(num),
            IntervalPeriod::Years(num) => self - chrono::Months::new(num * 12),
        }
    }
}

impl std::ops::AddAssign<IntervalPeriod> for NaiveDate {
    fn add_assign(&mut self, rhs: IntervalPeriod) {
        *self = *self + rhs
    }
}

impl std::ops::SubAssign<IntervalPeriod> for NaiveDate {
    fn sub_assign(&mut self, rhs: IntervalPeriod) {
        *self = *self - rhs
    }
}
