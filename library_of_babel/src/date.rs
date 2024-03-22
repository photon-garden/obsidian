use chrono::prelude::*;
use chrono::NaiveDate;
use std::fmt::Display;
use std::ops::RangeInclusive;

pub fn months() -> impl Iterator<Item = Month> {
    valid_months.map(Month::new)
}

pub fn current_year() -> Year {
    let year = Local::now().year();
    Year::new(year)
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Year(pub i32);

impl Year {
    pub fn new(year: i32) -> Year {
        Year(year)
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = format!("{:02}", self.0);
        write!(f, "{}", formatted)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Month(pub u8);

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = format!("{:02}", self.0);
        write!(f, "{}", formatted)
    }
}

impl Month {
    pub fn new(month: u8) -> Month {
        if !valid_months.contains(&month) {
            panic!("Month must be in the range 1 to 12.");
        }

        Month(month)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct DayOfTheMonth(pub u8);

impl Display for DayOfTheMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = format!("{:02}", self.0);
        write!(f, "{}", formatted)
    }
}

impl DayOfTheMonth {
    pub fn new(day_of_the_month: u8) -> DayOfTheMonth {
        if !valid_days_of_the_month.contains(&day_of_the_month) {
            panic!("Day of the month must be in the range 1 to 31.");
        }

        DayOfTheMonth(day_of_the_month)
    }
}

pub fn days_in_month(year: Year, month: Month) -> impl Iterator<Item = DayOfTheMonth> {
    let (year_for_next_date, month_for_next_date) = match month.0 {
        12 => {
            let year_for_next_date = year.0 + 1;
            let month_for_next_date = 1;
            (year_for_next_date, month_for_next_date)
        }
        _ => {
            let year_for_next_date = year.0;
            let month_for_next_date = month.0 + 1;
            (year_for_next_date, month_for_next_date)
        }
    };

    let next_date = NaiveDate::from_ymd_opt(year_for_next_date, month_for_next_date as u32, 1)
        .expect("Error creating next date.");

    let original_date =
        NaiveDate::from_ymd_opt(year.0, month.0 as u32, 1).expect("Error creating original date.");

    let num_days = next_date.signed_duration_since(original_date).num_days() as u8;

    (1..=num_days).map(DayOfTheMonth::new)
}

pub fn is_valid_month(number: u8) -> bool {
    valid_months.contains(&number)
}

pub fn is_valid_day_of_month(number: u8) -> bool {
    valid_days_of_the_month.contains(&number)
}

const valid_months: RangeInclusive<u8> = 1..=12;
const valid_days_of_the_month: RangeInclusive<u8> = 1..=31;
