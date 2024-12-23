use std::collections::HashMap;

use chrono::NaiveDate;

use super::CountryTwoCode;

#[derive(Default)]
pub struct NoCountryCodes;
#[derive(Default)]
pub struct CountryCodes(Vec<CountryTwoCode>);
#[derive(Default)]
pub struct NoPeriods;
#[derive(Default)]
pub struct Periods(Vec<u32>);

#[derive(Debug, Default)]
pub struct HolidaysBuilder<C, P> {
    country_codes: C,
    periods: P,
}
pub struct HolidaysRequest;
pub struct Holidays(HashMap<String, Vec<NaiveDate>>);

impl HolidaysBuilder<NoCountryCodes, NoPeriods> {
    fn new() -> Self {
        Self::default()
    }
}
