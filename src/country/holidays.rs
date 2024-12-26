use std::collections::HashMap;

use super::CountryTwoCode;
use chrono::NaiveDate;
use reqwest::blocking::Client;

//  --- Constants ---
const BASE_URL: &str = "https://date.nager.at/api/v3";
const RESOURCE_DIR: &str = "src/resources/holidays";

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
pub struct HolidaysRequest {
    client: Client,
    identifiers: Vec<String>,
    urls: Vec<String>,
}
pub struct Holidays<A>(HashMap<String, Vec<A>>);

impl HolidaysBuilder<NoCountryCodes, NoPeriods> {
    fn new() -> Self {
        Self::default()
    }
}

impl<C, P> HolidaysBuilder<C, P> {
    pub fn country_codes(
        self,
        country_codes: &[CountryTwoCode],
    ) -> HolidaysBuilder<CountryCodes, P> {
        HolidaysBuilder {
            country_codes: CountryCodes(country_codes.to_vec()),
            periods: self.periods,
        }
    }

    pub fn periods(self, periods: &[u32]) -> HolidaysBuilder<C, Periods> {
        HolidaysBuilder {
            country_codes: self.country_codes,
            periods: Periods(periods.to_vec()),
        }
    }
}

impl HolidaysBuilder<CountryCodes, Periods> {
    pub fn build(self) -> HolidaysRequest {
        let client = Client::new();

        let mut identifiers = Vec::new();
        let mut urls = Vec::new();

        for p in self.periods.0 {
            for c in self.country_codes.0.clone() {
                identifiers.push(c.clone().to_string());
                let url = format!("{}/PublicHolidays/{}/{}", BASE_URL, p, &c);
                urls.push(url);
            }
        }

        HolidaysRequest {
            client,
            identifiers,
            urls,
        }
    }
}

//  --- Cache ---
impl Holidays<u32> {}
