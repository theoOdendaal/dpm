use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub enum Periods {
    Days(u32),
    Weeks(u32),
    Months(u32),
    Years(u32),
}

impl std::ops::Add<Periods> for NaiveDate {
    type Output = NaiveDate;

    fn add(self, rhs: Periods) -> Self::Output {
        match rhs {
            Periods::Days(num) => self + chrono::Days::new(num as u64),
            Periods::Weeks(num) => self + chrono::Days::new(num as u64 * 7),
            Periods::Months(num) => self + chrono::Months::new(num),
            Periods::Years(num) => self + chrono::Months::new(num * 12),
        }
    }
}

impl std::ops::Sub<Periods> for NaiveDate {
    type Output = NaiveDate;

    fn sub(self, rhs: Periods) -> Self::Output {
        match rhs {
            Periods::Days(num) => self - chrono::Days::new(num as u64),
            Periods::Weeks(num) => self - chrono::Days::new(num as u64 * 7),
            Periods::Months(num) => self - chrono::Months::new(num),
            Periods::Years(num) => self - chrono::Months::new(num * 12),
        }
    }
}

impl std::ops::AddAssign<Periods> for NaiveDate {
    fn add_assign(&mut self, rhs: Periods) {
        *self = *self + rhs
    }
}

impl std::ops::SubAssign<Periods> for NaiveDate {
    fn sub_assign(&mut self, rhs: Periods) {
        *self = *self - rhs
    }
}
