use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

// FIXME refactor this code to make it more concise and understandable.

const BASE_URL: &str = "https://date.nager.at/api/v3";
const PUBLIC_HOLIDAYS_BASE: &str = "PublicHolidays";
const AVAILABLE_COUNTRIES_BASE: &str = "AvailableCountries";
const HOLIDAY_LIST: [&str; 2] = ["US", "ZA"];
const HOLIDAY_DIR: &str = "src/resources/holidays";

fn fetch_url(client: &Client, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = client.get(url).send()?;
    let response_text = response.text()?;
    Ok(response_text)
}

fn write_vec_to_txt(path: &str, text: Vec<String>) {
    let mut file = File::create(path).unwrap();
    for line in text.clone() {
        writeln!(file, "{}", line).unwrap();
    }
}

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

fn fetch_public_holidays(
    client: &Client,
    country_code: &str,
    period: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = &format!(
        "{}/{}/{}/{}",
        BASE_URL, PUBLIC_HOLIDAYS_BASE, period, country_code
    );
    let response_text = fetch_url(client, url)?;

    let public_holidays: Vec<PublicHoliday> = serde_json::from_str(&response_text)?;

    Ok(public_holidays
        .iter()
        .map(|holiday| holiday.date.clone())
        .collect())
}

fn fetch_available_countries(
    client: &Client,
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let url = format!("{}/{}", BASE_URL, AVAILABLE_COUNTRIES_BASE);
    let response_text = fetch_url(client, &url)?;

    let available_countries: Vec<AvailableCountries> = serde_json::from_str(&response_text)?;

    Ok(available_countries
        .iter()
        .map(|country| country.country_code.clone())
        .collect())
}

pub fn write_public_holidays(period: std::ops::RangeInclusive<u32>, country_code: &str) {
    let client = Client::new();
    let mut collection: Vec<String> = Vec::new();
    for n in period {
        if let Ok(text) = fetch_public_holidays(&client, country_code, &n.to_string()) {
            collection.extend(text);
        }
    }
    write_vec_to_txt(&format!("{}/{}.txt", HOLIDAY_DIR, country_code), collection);
}

pub fn load_country_calendar(
    country_code: &str,
) -> Result<HashSet<NaiveDate>, Box<dyn std::error::Error>> {
    let contents = read_to_string(format!("{}/{}.txt", HOLIDAY_DIR, country_code))?;
    let lines: HashSet<NaiveDate> = contents
        .lines()
        .map(|line| NaiveDate::parse_from_str(line, "%Y-%m-%d").unwrap())
        .collect();

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

pub fn update_holidays() {
    let period = 1990..=2025;
    for country in HOLIDAY_LIST {
        println!("Extracting: {:?}", country);
        write_public_holidays(period.clone(), country);
    }
}

pub fn update_with_all_available_holidays() {
    let period = 2024..=2025;
    let client = Client::new();
    if let Ok(holiday_list) = fetch_available_countries(&client) {
        for country in holiday_list {
            println!("Extracting: {:?}", country);
            write_public_holidays(period.clone(), &country);
        }
    }
}
