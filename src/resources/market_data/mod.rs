use std::collections::BTreeMap;

// TODO improve error handling.
// TODO better structure the resources module.

const BASE_PATH: &str = "src/resources/market_data";
const CURVE_PATH: &str = "curves";
const SPOT_PATH: &str = "spot";

//  --- Errors ---
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    SerializationError(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "{}", err),
            Self::SerializationError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializationError(value)
    }
}

//  --- Standalone functions

fn load_data<A, B>(dir: &str) -> Result<BTreeMap<A, B>>
where
    A: serde::de::DeserializeOwned + Ord,
    B: serde::de::DeserializeOwned,
{
    let contents = std::fs::read_to_string(dir)?;
    let contents: BTreeMap<A, B> = serde_json::from_str(&contents)?;
    Ok(contents)
}

pub fn load_curve<A, B>(name: &str) -> Result<BTreeMap<A, B>>
where
    A: serde::de::DeserializeOwned + Ord,
    B: serde::de::DeserializeOwned,
{
    let dir = format!("{}/{}/{}.txt", BASE_PATH, CURVE_PATH, name);
    load_data(&dir)
}

pub fn load_spot<A, B>(name: &str) -> Result<BTreeMap<A, B>>
where
    A: serde::de::DeserializeOwned + Ord,
    B: serde::de::DeserializeOwned,
{
    let dir = format!("{}/{}/{}.txt", BASE_PATH, SPOT_PATH, name);
    load_data(&dir)
}
