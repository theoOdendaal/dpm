use chrono::{Datelike, Days, Months, NaiveDate};

//  --- Errors

//  --- Traits
// TODO move to more appropriate module!
pub trait EndOfMonth: Sized {
    fn eom(&self) -> Option<Self>;
}

impl EndOfMonth for NaiveDate {
    fn eom(&self) -> Option<Self> {
        self.checked_sub_days(Days::new(self.day() as u64))
            .and_then(|d| d.checked_add_months(Months::new(1)))
    }
}

//  --- Trait implementations: Blanket
impl<A> EndOfMonth for Vec<A>
where
    A: EndOfMonth,
{
    fn eom(&self) -> Option<Self> {
        self.iter().map(|d| d.eom()).collect()
    }
}
