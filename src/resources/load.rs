use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

// FIXME refactor this code to make it more concise and understandable.

const PUBLIC_HOLIDAYS_BASE: &str = "https://date.nager.at/api/v3/PublicHolidays";
const AVAILABLE_COUNTRIES_BASE: &str = "https://date.nager.at/api/v3/AvailableCountries";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PublicHoliday {
    date: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AvailableCountries {
    country_code: String,
}

const HOLIDAY_LIST: [&str; 2] = ["US", "ZA"];

fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send()?;
    let response_text = response.text()?;
    Ok(response_text)
}

pub fn load_holidays(country_code: &str) -> Result<HashSet<NaiveDate>, Box<dyn std::error::Error>> {
    let contents = read_to_string(format!("{}/{}.txt", "src/resources/holidays", country_code))?;

    // Split the string into lines and collect them into a Vec<String>
    let lines: HashSet<NaiveDate> = contents
        .lines()
        .map(|line| NaiveDate::parse_from_str(line, "%Y-%m-%d").unwrap())
        .collect();

    Ok(lines)
}

pub fn update_holidays() {
    let period = 2024..=2025;
    for country in HOLIDAY_LIST {
        write_holiday_range(period.clone(), country);
    }
}

fn write_holiday_range(period: std::ops::RangeInclusive<u32>, country_code: &str) {
    let mut holiday_collection: Vec<String> = Vec::new();

    for n in period {
        if let Ok(dates) = fetch_public_holidays(&n.to_string(), country_code) {
            for holiday in dates.iter() {
                holiday_collection.push(holiday.clone());
            }
        }
    }

    let path = format!("{}/{}.txt", "src/resources/holidays", country_code);
    let mut file = File::create(path).unwrap();
    for date in holiday_collection.clone() {
        writeln!(file, "{}", date).unwrap();
    }
}

pub fn fetch_public_holidays(
    period: &str,
    country_code: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = &format!("{}/{}/{}", PUBLIC_HOLIDAYS_BASE, period, country_code);
    let response_text = fetch(url)?;

    let public_holidays: Vec<PublicHoliday> = serde_json::from_str(&response_text)?;

    Ok(public_holidays
        .iter()
        .map(|holiday| holiday.date.clone())
        .collect())
}

pub fn fetch_available_countries() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = AVAILABLE_COUNTRIES_BASE;
    let response_text = fetch(url)?;

    let available_countries: Vec<AvailableCountries> = serde_json::from_str(&response_text)?;

    Ok(available_countries
        .iter()
        .map(|country| country.country_code.clone())
        .collect())
}
