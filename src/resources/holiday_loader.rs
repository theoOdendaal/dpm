use chrono::{Datelike, NaiveDate};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;

type Result<T> = std::result::Result<T, Error>;

// https://date.nager.at/swagger/index.html

// FIXME refactor this code to make it more concise and understandable.
// TODO create a config file, thats stores all available country codes and periods.

pub const BASE_URL: &str = "https://date.nager.at/api/v3";
pub const HOLIDAY_LIST: [&str; 2] = ["US", "ZA"];
pub const HOLIDAY_DIR: &str = "src/resources/holidays";

//  --- Errors ---
#[derive(Debug)]
pub enum Error {
    UnknownCountryCode,
    PeriodNotAvailable,
    UnanticipatedError,
    IOError(std::io::Error),
    FetchError(reqwest::Error),
    ParseError(serde_json::Error),
    DateParseError(chrono::format::ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCountryCode => write!(f, "Unknown country code."),
            Self::PeriodNotAvailable => write!(f, "The requested period is not available."),
            Self::UnanticipatedError => write!(f, "An unknown error occurred."),
            Self::IOError(err) => write!(f, "{}", err),
            Self::FetchError(err) => write!(f, "{}", err),
            Self::ParseError(err) => write!(f, "{}", err),
            Self::DateParseError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::FetchError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(value: chrono::format::ParseError) -> Self {
        Self::DateParseError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::ParseError(value)
    }
}

//  --- Utility ---

pub fn fetch_url(client: &Client, url: &str) -> Result<String> {
    let response = client.get(url).send()?;
    match response.status() {
        StatusCode::OK => Ok(response.text()?), // 200
        StatusCode::BAD_REQUEST => Err(Error::PeriodNotAvailable), // 400
        StatusCode::NOT_FOUND => Err(Error::UnknownCountryCode), // 404
        _ => Err(Error::UnanticipatedError),
    }
}

fn write_to_file(path: &str, text: Vec<String>) -> Result<()> {
    let path = std::path::Path::new(path);
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir_all(path.parent().unwrap())?;
    }

    let mut file = File::create(path)?;
    for line in text.clone() {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

//  --- Deserialization structs ---

#[derive(Deserialize, Debug)]
struct PublicHoliday {
    date: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AvailableCountries {
    country_code: String,
}

//  --- Functionality ---

fn fetch_public_holidays(client: &Client, country_code: &str, period: &str) -> Result<Vec<String>> {
    let url = &format!("{}/PublicHolidays/{}/{}", BASE_URL, period, country_code);

    let response_text = fetch_url(client, url)?;

    let public_holidays: Vec<PublicHoliday> = serde_json::from_str(&response_text)?;
    Ok(public_holidays.into_iter().map(|h| h.date).collect())
}

fn fetch_available_countries(client: &Client) -> Result<HashSet<String>> {
    let url = format!("{}/AvailableCountries", BASE_URL);
    let response_text = fetch_url(client, &url)?;
    let available_countries: Vec<AvailableCountries> = serde_json::from_str(&response_text)?;

    Ok(available_countries
        .into_iter()
        .map(|c| c.country_code)
        .collect())
}

pub fn write_public_holidays(
    client: &Client,
    period: std::ops::RangeInclusive<u32>,
    country_code: &str,
) -> Result<()> {
    let mut collection: Vec<String> = Vec::new();
    for n in period {
        if let Ok(text) = fetch_public_holidays(client, country_code, &n.to_string()) {
            collection.extend(text);
        }
    }
    write_to_file(&format!("{}/{}.txt", HOLIDAY_DIR, country_code), collection)?;

    Ok(())
}

pub fn read_country_calendar(country_code: &str) -> Result<HashSet<NaiveDate>> {
    let contents = fs::read_to_string(format!("{}/{}.txt", HOLIDAY_DIR, country_code))?;
    let lines: HashSet<NaiveDate> = contents
        .lines()
        .map(|line| NaiveDate::parse_from_str(line, "%Y-%m-%d").map_err(Error::DateParseError))
        .collect::<Result<HashSet<NaiveDate>>>()?;

    Ok(lines)
}

pub fn read_saved_holidays_population() -> HashSet<String> {
    let mut txt_files = HashSet::new();

    if let Ok(entries) = fs::read_dir(HOLIDAY_DIR) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
                if let Some(file_stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                    txt_files.insert(file_stem.to_string());
                }
            }
        }
    }

    txt_files
}

pub fn update_holidays() -> Result<()> {
    let client = Client::new();
    let period = 2020..=2025;
    for country in HOLIDAY_LIST {
        println!("Extracting: {:?}", country);
        write_public_holidays(&client, period.clone(), country)?;
    }
    Ok(())
}

pub fn update_with_all_available_holidays() -> Result<()> {
    let client = Client::new();
    let period = 2024..=2025;
    if let Ok(holiday_list) = fetch_available_countries(&client) {
        for country in holiday_list {
            println!("Extracting: {:?}", country);
            write_public_holidays(&client, period.clone(), &country)?;
        }
    }
    Ok(())
}

//  --- config ---
// TODO, all data written to txt should go through this struct.
// FIXME Data written should be appended and not overwritten.
#[derive(Clone, Debug)]
pub struct HolidayLoader(HashMap<String, HashSet<i32>>);

impl HolidayLoader {
    pub fn new() -> Self {
        let mut values = HashMap::new();
        let countries = read_saved_holidays_population();
        for country in countries.into_iter() {
            if let Ok(calendar) = read_country_calendar(&country) {
                let calendar: HashSet<i32> = calendar.iter().map(|x| x.year()).collect();
                values.insert(country, calendar);
            }
        }
        Self(values)
    }

    pub fn is_available(&self, country: &str, period: i32) -> bool {
        if let Some(dates) = self.0.get(country) {
            dates.contains(&period)
        } else {
            false
        }
    }

    pub fn load_if_not_available(
        &mut self,
        country: &str,
        period: i32,
    ) -> Result<HashSet<NaiveDate>> {
        if !self.is_available(country, period) {
            write_public_holidays(&Client::new(), (period as u32)..=(period as u32), country)?;
            *self = Self::new();
        }
        read_country_calendar(country)
    }
}

impl Default for HolidayLoader {
    fn default() -> Self {
        Self::new()
    }
}
