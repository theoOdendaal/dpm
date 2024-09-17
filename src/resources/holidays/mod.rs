use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::{fmt::Debug, fs::File};

// https://date.nager.at/swagger/index.html

// Refactor to make it more concise.

//  --- Constants ---
const BASE_URL: &str = "https://date.nager.at/api/v3";
const RESOURCE_DIR: &str = "src/resources/holidays";

//  --- Errors ---
type Result<T> = std::result::Result<T, Error>;

// TODO create more specific variants.
#[derive(Debug)]
pub enum Error {
    Static(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Static(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Static(value.to_string())
    }
}

impl From<chrono::ParseError> for Error {
    fn from(value: chrono::format::ParseError) -> Self {
        Self::Static(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Static(value.to_string())
    }
}

//  --- Structs ---

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

#[derive(Debug)]
pub struct PublicHolidayRequest {
    client: Client,
    identifiers: Vec<String>,
    urls: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PublicHoliday {
    date: String,
}

#[derive(Debug)]
pub struct HolidayCalendar(HashMap<String, HashSet<NaiveDate>>);

//  --- Custom implementations ---

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
    pub fn fetch(&self) -> Result<HolidayCalendar> {
        let mut holidays: HashMap<String, HashSet<NaiveDate>> = HashMap::new();

        for (cc, url) in self.identifiers.iter().zip(self.urls.iter()) {
            let fetched_holidays = fetch_url(&self.client, url)?;

            let json_text: Vec<PublicHoliday> = serde_json::from_str(&fetched_holidays)?;

            let parsed_holidays: HashSet<NaiveDate> = json_text
                .iter()
                .map(NaiveDate::try_from)
                .collect::<Result<HashSet<NaiveDate>>>()?;

            holidays
                .entry(cc.to_string())
                .and_modify(|dates| dates.extend(parsed_holidays.clone()))
                .or_insert(parsed_holidays);
        }
        Ok(holidays.into())
    }
}

impl HolidayCalendar {
    pub fn save(&self) -> Result<Self> {
        for (k, v) in self.0.iter() {
            write_to_file(k, v)?;
        }
        Ok(Self(self.0.clone()))
    }
}

//  --- Standard library trait implementations ---

impl From<HashMap<String, HashSet<NaiveDate>>> for HolidayCalendar {
    fn from(value: HashMap<String, HashSet<NaiveDate>>) -> Self {
        Self(value)
    }
}

impl From<HolidayCalendar> for HashMap<String, HashSet<NaiveDate>> {
    fn from(value: HolidayCalendar) -> Self {
        value.0
    }
}

impl TryFrom<&PublicHoliday> for NaiveDate {
    type Error = Error;

    fn try_from(value: &PublicHoliday) -> std::result::Result<Self, Self::Error> {
        NaiveDate::parse_from_str(&value.date, "%Y-%m-%d")
            .map_err(|e| Error::Static(format!("Failed to parse date: {}", e)))
    }
}

//  --- Utility functions ---

fn fetch_url(client: &Client, url: &str) -> Result<String> {
    // TODO match on status codes.
    Ok(client.get(url).send()?.text()?)
}

//  --- Functionality ---

fn write_to_file<T>(file_name: &str, collection: &HashSet<T>) -> Result<()>
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

pub fn load_holidays(country_code: &str) -> Result<HashSet<NaiveDate>> {
    let path = format!("{}/{}.txt", RESOURCE_DIR, country_code);
    let contents = fs::read_to_string(path)?;
    //contents.lines().map(|d| NaiveDate::from(d)).collect()
    //into_date_collection(contents.lines().map(|d| d.to_string()).collect())
    contents
        .lines()
        .map(|d| {
            NaiveDate::parse_from_str(d, "%Y-%m-%d")
                .map_err(|e| Error::Static(format!("Failed to parse date: {}", e)))
        })
        .collect::<Result<HashSet<NaiveDate>>>()
}
