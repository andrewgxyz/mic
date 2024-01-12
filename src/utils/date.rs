use chrono::{DateTime, Utc, NaiveDate, ParseError, Datelike, Weekday, Local};

pub fn parse_string_to_datetime(input: &str) -> Result<DateTime<Utc>, ParseError> {
    let naive_date = NaiveDate::parse_from_str(input, "%Y-%m-%d")?;
    let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc))
}

pub fn parse_string_to_yearless_date(input: &str) -> NaiveDate {
    let naive_date = NaiveDate::parse_from_str(input, "%Y-%m-%d").unwrap();
    let yearless_date = NaiveDate::from_ymd_opt(0, naive_date.month(), naive_date.day()).unwrap();

    yearless_date
}

pub fn get_start_end_week_dates() -> (NaiveDate, NaiveDate) {
    let dt: DateTime<Local> = chrono::offset::Local::now();
    let naive_date = NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap();
    let week = naive_date.week(Weekday::Sat);

    (week.first_day(), week.last_day())
}
