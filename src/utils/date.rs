use chrono::{DateTime, Utc, NaiveDate, ParseError, Datelike};

pub fn parse_string_to_datetime(input: &str) -> Result<DateTime<Utc>, ParseError> {
    let naive_date = NaiveDate::parse_from_str(input, "%Y-%m-%d")?;
    let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc))
}
