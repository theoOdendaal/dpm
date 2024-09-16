use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::{fmt::Debug, fs::File};

//  --- Constants ---
const BASE_URL: &str = "https://date.nager.at/api/v3";
const RESOURCE_DIR: &str = "src/resources/holidays";

//  --- Structs ---

#[derive(Debug)]
pub struct PublicHolidayRequest {
    client: Client,
    identifiers: Vec<String>,
    urls: Vec<String>,
}

#[derive(Default, Clone)]
pub struct NoCountryCodes;
#[derive(Default, Clone)]
pub struct CountryCodes(HashSet<String>);

#[derive(Default, Clone)]
pub struct NoPeriods;
#[derive(Default, Clone)]
pub struct Periods(HashSet<u32>);

#[derive(Default, Clone)]
pub struct PublicHolidayRequestBuilder<C, P> {
    country_codes: C,
    periods: P,
}

#[derive(Deserialize, Debug)]
pub struct PublicHoliday {
    date: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvailableCountries {
    country_code: String,
}

//  --- Implementations ---

impl PublicHolidayRequestBuilder<NoCountryCodes, NoPeriods> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<C, P> PublicHolidayRequestBuilder<C, P> {
    pub fn country_codes(self, countries: &[&str]) -> PublicHolidayRequestBuilder<CountryCodes, P> {
        PublicHolidayRequestBuilder {
            country_codes: CountryCodes(countries.iter().map(|d| d.to_string()).collect()),
            periods: self.periods,
        }
    }

    pub fn periods(self, periods: &[u32]) -> PublicHolidayRequestBuilder<C, Periods> {
        PublicHolidayRequestBuilder {
            country_codes: self.country_codes,
            periods: Periods(periods.iter().copied().collect()),
        }
    }
}

impl PublicHolidayRequestBuilder<CountryCodes, Periods> {
    pub fn build(self) -> PublicHolidayRequest {
        let client = Client::new();

        let mut identifiers = Vec::new();
        let mut urls = Vec::new();

        for p in self.periods.0 {
            for c in self.country_codes.0.clone() {
                identifiers.push(c.clone());
                let url = format!("{}/PublicHolidays/{}/{}", BASE_URL, p, &c);
                urls.push(url);
            }
        }

        PublicHolidayRequest {
            client,
            identifiers,
            urls,
        }
    }
}

impl PublicHolidayRequest {
    pub fn fetch(self) -> Result<HashMap<String, Vec<String>>, reqwest::Error> {
        let mut holidays: HashMap<String, Vec<String>> = HashMap::new();

        for (cc, url) in self.identifiers.into_iter().zip(self.urls.iter()) {
            let fetched_holidays = fetch_url(&self.client, url)?;
            let parsed_holidays = into_public_holidays(fetched_holidays).unwrap();
            holidays
                .entry(cc)
                .and_modify(|dates| dates.extend(parsed_holidays.clone()))
                .or_insert(parsed_holidays);
        }
        Ok(holidays)
    }
}

//  --- Utility ---

// TODO match on status codes.
fn fetch_url(client: &Client, url: &str) -> Result<String, reqwest::Error> {
    client.get(url).send()?.text()
}

//  --- Functionality ---

fn into_public_holidays(text: String) -> Result<Vec<String>, serde_json::Error> {
    let json_text: Vec<PublicHoliday> = serde_json::from_str(&text)?;
    Ok(json_text.iter().map(|d| d.date.clone()).collect())
}

fn into_available_countries(text: String) -> Result<HashSet<String>, serde_json::Error> {
    serde_json::from_str(&text)
}

fn write_to_file<T>(file_name: &str, collection: Vec<T>) -> Result<(), std::io::Error>
where
    T: Debug,
{
    let path = format!("{}/{}.txt", RESOURCE_DIR, file_name);
    let mut file = File::create(path)?;
    for line in collection {
        writeln!(file, "{:?}", line)?
    }
    Ok(())
}

fn load_public_holidays(country_code: &str) -> Result<HashSet<String>, std::io::Error> {
    let path = format!("{}/{}.txt", RESOURCE_DIR, country_code);
    let contents = fs::read_to_string(path)?;
    let lines: HashSet<String> = contents.lines().map(|d| d.to_string()).collect();
    Ok(lines)
}
