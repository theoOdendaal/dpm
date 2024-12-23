use std::collections::HashMap;

use chrono::NaiveDate;

use super::CountryTwoCode;

pub struct NoCountryCodes;
pub struct CountryCodes(Vec<CountryTwoCode>);
pub struct NoPeriods;
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
        Self::defailt
    }
}

impl<C, P> for HolidaysBuilder<C, P> {
    fn new() {
        Self::defail
        
    }
}