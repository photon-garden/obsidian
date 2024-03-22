use chrono::{Datelike, NaiveDate};
use once_cell::sync::Lazy;

pub static POEMS: Lazy<Vec<Haiku>> = Lazy::new(load);

#[derive(Debug, serde::Deserialize)]
pub struct UnparsedHaiku {
    pub text: String,
    pub date: String,
}

#[derive(Debug)]
pub struct Haiku {
    pub text: String,
    pub date: NaiveDate,
}

impl Haiku {
    fn distance_from_target_date_ignoring_year(&self, target_date: &NaiveDate) -> i64 {
        // If self.date is Feb 29th in a leap year
        // and target_date is not a leap year,
        let should_correct_for_leap_year =
            self.date.month() == 2 && self.date.day() == 29 && !target_date.leap_year();

        // If we failed to correct for leap years, instantiating self_date_with_target_year
        // below would panic in the case where self.date is Feb 29th in a leap year and
        // target_date isn't a leap year.
        let day = if should_correct_for_leap_year {
            28 // Since February 29th doesn't exist in the target year, use February 28th.
        } else {
            self.date.day()
        };

        let self_date_with_target_year =
            NaiveDate::from_ymd_opt(target_date.year(), self.date.month(), day)
                .expect("Failed to create date.");

        (self_date_with_target_year - *target_date).num_days().abs()
    }
}

pub fn find_closest_poems(date: &NaiveDate) -> Vec<&'static Haiku> {
    let closest_distance_in_days = POEMS
        .iter()
        .map(|haiku| haiku.distance_from_target_date_ignoring_year(date))
        .min()
        .unwrap();

    POEMS
        .iter()
        .filter(|haiku| {
            haiku.distance_from_target_date_ignoring_year(date) == closest_distance_in_days
        })
        .collect()
}

fn load() -> Vec<Haiku> {
    let csv_string = include_str!("processed_2023_haiku.csv");
    let unparsed_poems = csv::Reader::from_reader(csv_string.as_bytes())
        .deserialize()
        .collect::<Result<Vec<UnparsedHaiku>, _>>()
        .expect("Failed to parse CSV.");

    unparsed_poems
        .into_iter()
        .map(|unparsed_poem| {
            let date = NaiveDate::parse_from_str(&unparsed_poem.date, "%Y-%m-%d")
                .expect("Failed to parse date.");
            Haiku {
                text: unparsed_poem.text,
                date,
            }
        })
        .collect()
}
