#[derive(Debug)]
pub enum Errors {
    CountryCodeParsing(String),
}

#[derive(Debug)]
pub struct CountryCodeParsing(pub String);

impl std::fmt::Display for CountryCodeParsing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse {} as CountryCode.", self.0)
    }
}

impl std::error::Error for CountryCodeParsing {}
