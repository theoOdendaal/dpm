use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

const BASE_HOLIDAYS_URL: &str = "https://date.nager.at/api/v3/PublicHolidays";

const HOLIDAY_LIST: [&str; 2] = ["US", "ZA"];

pub fn load_holidays(country_code: &str) -> Result<Vec<NaiveDate>, Box<dyn std::error::Error>> {
    let contents = read_to_string(format!("{}/{}.txt", "src/resources/holidays", country_code))?;

    // Split the string into lines and collect them into a Vec<String>
    let lines: Vec<NaiveDate> = contents
        .lines()
        .map(|line| NaiveDate::parse_from_str(line, "%Y-%m-%d").unwrap())
        .collect();

    Ok(lines)
}

pub fn update_holidays() {
    let period = 2020..=2025;
    for country in HOLIDAY_LIST {
        write_holiday_range(period.clone(), country);
    }
}

fn write_holiday_range(period: std::ops::RangeInclusive<u32>, country_code: &str) {
    let mut holiday_collection: Vec<String> = Vec::new();

    for n in period {
        if let Ok(dates) = fetch_holidays(&n.to_string(), country_code) {
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PublicHoliday {
    date: String,
}

fn fetch_holidays(
    period: &str,
    country_code: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!("{}/{}/{}", BASE_HOLIDAYS_URL, period, country_code);

    let client = Client::new();
    let response = client.get(&url).send()?;

    let response_text = response.text()?;

    let holidays: Vec<PublicHoliday> = serde_json::from_str(&response_text)?;

    Ok(holidays
        .iter()
        .map(|holiday| holiday.date.clone())
        .collect())
}
