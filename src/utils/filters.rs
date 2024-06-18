use chrono::{DateTime, Utc, Datelike, NaiveDate};

use super::{date::{get_start_end_week_dates, parse_string_to_yearless_date}, data::string_clean};

pub fn contains_list_of_strings (needles: &Option<String>, haystack: &Vec<String>) -> bool {
    match needles {
        Some(string) => string
            .split(',')
            .map(|m| m.to_string())
            .any(|needle| haystack.contains(&needle)),
        None => true
    }
}

pub fn match_current_week(release: &String) -> bool {
    let (sat, fri) = get_start_end_week_dates();
    let (yearless_sat, yearless_fri) = (
        NaiveDate::from_ymd_opt(0, sat.month(), sat.day()).unwrap(),
        NaiveDate::from_ymd_opt(0, fri.month(), fri.day()).unwrap(),
    );
    let yearless_date = parse_string_to_yearless_date(release);

    yearless_date.ge(&yearless_sat) && yearless_date.le(&yearless_fri)
}

pub fn match_items_left(dt: &str) -> bool {
    let yearless_dt = parse_string_to_yearless_date(dt);
    let yearless_today = parse_string_to_yearless_date(&(chrono::offset::Local::now().to_string()));

    yearless_dt >= yearless_today
}

pub fn match_decade(dt: DateTime<Utc>, decade: &Option<u16>) -> bool {
    let year = dt.year();
    let decade_year = ((year / 10) * 10) as u16;

    equals_same_value::<u16>(decade, &decade_year)
}

pub fn match_no_lyrics(lyrics: &Vec<String>) -> bool {
    lyrics.len() == 0
}

pub fn match_lyrics_contain_words(words: &Option<String>, lyrics: &Vec<String>) -> bool {
    match words {
        Some(words) => {
            words.split(',').map(|w| w.to_lowercase().to_string()).any(|word| {
                let mut has_word = false;
                for phrases in lyrics.clone() {
                    let cleaned = string_clean(phrases);

                    if cleaned.contains(&word) {
                        has_word = true;
                    }
                }

                has_word
            })
        },
        None => true
    }
}

pub fn equals_same_value<T>(needle: &Option<T>, haystack: &T) -> bool 
where T: PartialEq<T> {
    match needle {
        Some(val) => val == haystack,
        None => true
    }
}
