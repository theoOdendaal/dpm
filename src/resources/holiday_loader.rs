use chrono::NaiveDate;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;

type Result<T> = std::result::Result<T, Error>;

// https://date.nager.at/swagger/index.html

// FIXME refactor this code to make it more concise and understandable.
// TODO implement custom errors.
// TODO replace all .unwraps with proper error propogation.
// FIXME don't use Box<dyn Error>, as this defines behaviour at runtime.
// Imple the From trait for all Error's raised by other crates utilized.

pub const BASE_URL: &str = "https://date.nager.at/api/v3";
pub const HOLIDAY_LIST: [&str; 2] = ["US", "ZA"];
pub const HOLIDAY_DIR: &str = "src/resources/holidays";

//  --- Errors ---
#[derive(Debug)]
pub enum Error {
    UnknownCountryCode,
    PeriodNotAvailable,
    UnanticipatedError,
    IOError(String),
    WriteError(String),
    FetchError(String),
    ResponseError(String),
    ParseError(String),
    ReadError(String),
    DateParseError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCountryCode => write!(f, "Unknown country code."),
            Self::PeriodNotAvailable => write!(f, "The requested period is not available."),
            Self::UnanticipatedError => write!(f, "An unknown error occurred."),
            Self::IOError(err) => write!(f, "{}", err),
            Self::WriteError(err) => write!(f, "Unable to write file for country code: {}.", err),
            Self::FetchError(err) => write!(f, "Unable to fetch data for country code: {}.", err),
            Self::ResponseError(err) => write!(f, "{}.", err),
            Self::ParseError(err) => write!(f, "Unable to parse data for country code: {}.", err),
            Self::ReadError(err) => write!(f, "Unable to read file for country code: {}.", err),
            Self::DateParseError(err) => write!(f, "Failed to parse date from string: {}.", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::FetchError(value.to_string())
    }
}

// FIXME refactor this code. Make it more concise.
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.to_string())
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
    let mut file = File::create(path)?;
    for line in text.clone() {
        writeln!(file, "{}", line).map_err(|_| Error::WriteError(path.into()))?;
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

pub fn fetch_public_holidays(
    client: &Client,
    country_code: &str,
    period: &str,
) -> Result<Vec<String>> {
    let url = &format!("{}/PublicHolidays/{}/{}", BASE_URL, period, country_code);

    // FIXME match on the status code returned.
    let response_text =
        fetch_url(client, url).map_err(|_| Error::FetchError(country_code.into()))?;

    let public_holidays: Vec<PublicHoliday> =
        serde_json::from_str(&response_text).map_err(|_| Error::ParseError(country_code.into()))?;
    Ok(public_holidays.into_iter().map(|h| h.date).collect())
}

pub fn fetch_available_countries(client: &Client) -> Result<HashSet<String>> {
    let url = format!("{}/AvailableCountries", BASE_URL);
    let response_text =
        fetch_url(client, &url).map_err(|_| Error::FetchError("All available".into()))?;
    let available_countries: Vec<AvailableCountries> = serde_json::from_str(&response_text)
        .map_err(|_| Error::ParseError("All available".into()))?;

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
    write_to_file(&format!("{}/{}.txt", HOLIDAY_DIR, country_code), collection)
        .map_err(|_| Error::WriteError(country_code.into()))?;

    Ok(())
}

pub fn load_country_calendar(country_code: &str) -> Result<HashSet<NaiveDate>> {
    let contents = fs::read_to_string(format!("{}/{}.txt", HOLIDAY_DIR, country_code))
        .map_err(|_| Error::ParseError(country_code.into()))?;
    let lines: HashSet<NaiveDate> = contents
        .lines()
        .map(|line| {
            NaiveDate::parse_from_str(line, "%Y-%m-%d")
                .map_err(|_| Error::DateParseError(line.into()))
        })
        .collect::<Result<HashSet<NaiveDate>>>()?;

    Ok(lines)
}

pub fn load_saved_holidays_population() -> HashSet<String> {
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
    let period = 1990..=2025;
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
