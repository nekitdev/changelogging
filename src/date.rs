use time::{error::Parse as ParseError, macros::format_description, Date, OffsetDateTime};

pub type Error = ParseError;

pub fn today() -> Date {
    OffsetDateTime::now_utc().date()
}

pub fn parse(string: &str) -> Result<Date, Error> {
    let description = format_description!("[year]-[month]-[day]");

    Date::parse(string, description)
}
