//! # Holiday API Fetcher
//!
//! This module provides functionality for fetching public holiday data from the Nager.Date API,
//! handling the data in a local cache, and managing error handling for requests, file operations,
//! and serialization.
//!
//! ## Example
//!
//! The following example demonstrates how to fetch public holidays for specified countries and
//! save the results locally:
//!
//! ```rust,ignore
//! use chrono::NaiveDate;
//! use holidays::PublicHolidayRequestBuilder;
//!
//!let country_codes: Vec<&str> = vec!["ZA", "US"];
//!let periods: Vec<u32> = vec![2023, 2024];
//!
//!let country_calendar = PublicHolidayRequestBuilder::new()
//!     .country_codes(&country_codes)
//!     .periods(&periods)
//!     .build()
//!     .fetch()
//!     .unwrap()
//!     .save()
//!     .unwrap();
//!
//!
//!let za_holidays = holiday_api::load_holidays("ZA").unwrap();
//!
//!println!("Holidays in South Africa: {:?}", za_holidays);
//! ```
//! Ensure that you have an internet connection for the API request, and the saved holidays
//! will be written to the `src/resources/holidays` directory as text files.

use chrono::NaiveDate;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::{fmt::Debug, fs::File};

// https://date.nager.at/swagger/index.html

// TODO don't fetch when data is already downloaded? Create a config that json which stores search paramters?
// TODO improve Error enum.
// TODO Refactor to make it more concise.

//  --- Constants ---
const BASE_URL: &str = "https://date.nager.at/api/v3";
const RESOURCE_DIR: &str = "src/resources/holidays";

//  --- Errors ---
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    ResponseError(reqwest::StatusCode),
    IOError(std::io::Error),
    ParseError(chrono::format::ParseError),
    SerializationError(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError(err) => write!(f, "{}", err),
            Self::ResponseError(err) => write!(f, "{}", err),
            Self::IOError(err) => write!(f, "{}", err),
            Self::ParseError(err) => write!(f, "{:?}", err),
            Self::SerializationError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(value: chrono::format::ParseError) -> Self {
        Self::ParseError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializationError(value)
    }
}

//  --- Structs ---

#[derive(Default, Clone)]
struct NoCountryCodes;

#[derive(Default, Clone)]
pub struct CountryCodes(Vec<String>);

#[derive(Default, Clone)]
struct NoPeriods;

#[derive(Default, Clone)]
pub struct Periods(Vec<u32>);

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
struct PublicHoliday {
    date: NaiveDate,
}

#[derive(Debug)]
pub struct HolidayCalendar(HashMap<String, Vec<NaiveDate>>);

//  --- Implementations: Custom traits ---

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
            periods: Periods(periods.to_vec()),
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
        let mut holidays: HashMap<String, Vec<NaiveDate>> = HashMap::new();

        for (cc, url) in self.identifiers.iter().zip(self.urls.iter()) {
            let fetched_holidays = fetch_url(&self.client, url)?;

            let json_text: Vec<PublicHoliday> = serde_json::from_str(&fetched_holidays)?;
            let parsed_holidays: Vec<NaiveDate> = json_text.iter().map(|d| d.into()).collect();

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

//  --- Implementations: Standard library traits ---

impl From<HashMap<String, Vec<NaiveDate>>> for HolidayCalendar {
    fn from(value: HashMap<String, Vec<NaiveDate>>) -> Self {
        Self(value)
    }
}

impl From<HolidayCalendar> for HashMap<String, Vec<NaiveDate>> {
    fn from(value: HolidayCalendar) -> Self {
        value.0
    }
}

impl From<&PublicHoliday> for NaiveDate {
    fn from(value: &PublicHoliday) -> Self {
        value.date
    }
}

//  --- Utility functions ---

fn fetch_url(client: &Client, url: &str) -> Result<String> {
    // TODO match on status codes.
    let response = client.get(url).send()?;
    match response.status() {
        StatusCode::OK => Ok(response.text()?), // 200
        status => Err(Error::ResponseError(status)),
    }
}

//  --- Functionality ---

fn write_to_file(file_name: &str, collection: &Vec<NaiveDate>) -> Result<()> {
    let path = format!("{}/{}.txt", RESOURCE_DIR, file_name);
    let mut file = File::create(path)?;
    let json_string = serde_json::to_string(collection)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

pub fn load_holidays(country_code: &str) -> Result<Vec<NaiveDate>> {
    let path = format!("{}/{}.txt", RESOURCE_DIR, country_code);
    let contents = fs::read_to_string(path)?;
    let json_text: Vec<NaiveDate> = serde_json::from_str(&contents)?;
    Ok(json_text)
}
