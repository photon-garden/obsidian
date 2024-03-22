use std::collections::HashMap;

use chrono::{Datelike, NaiveDate};
use once_cell::sync::Lazy;

type Year = i32;

static SUN_STATIONS: Lazy<HashMap<Year, SunStationsForYear>> = Lazy::new(load_sun_stations);

pub fn get_closest_upcoming_solstice_or_equinox() -> SunStationWithDate {
    let today = chrono::Local::now().date_naive();

    let this_year = today.year();
    let next_year = this_year + 1;
    let sun_stations_for_this_year = SUN_STATIONS.get(&this_year).unwrap();
    let sun_stations_for_next_year = SUN_STATIONS.get(&next_year).unwrap();

    let all_sun_stations = [
        sun_stations_for_this_year.all(),
        sun_stations_for_next_year.all(),
    ]
    .concat();

    let (closest_future_sun_station, _distance_in_days) = all_sun_stations
        .into_iter()
        .map(|sun_station| (sun_station, sun_station.distance_in_days_from(today)))
        .filter(|(_sun_station, distance_in_days)| distance_in_days >= &0) // Only consider future sun stations.
        .min_by_key(|(_sun_station, distance_in_days)| *distance_in_days)
        .unwrap();

    closest_future_sun_station.clone()
}

struct SunStationsForYear {
    march_equinox: SunStationWithDate,
    june_solstice: SunStationWithDate,
    september_equinox: SunStationWithDate,
    december_solstice: SunStationWithDate,
}

impl SunStationsForYear {
    fn all(&self) -> Vec<&SunStationWithDate> {
        vec![
            &self.march_equinox,
            &self.june_solstice,
            &self.september_equinox,
            &self.december_solstice,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct SunStationWithDate {
    date: NaiveDate,
    pub kind: SunStationKind,
}

impl SunStationWithDate {
    /// Positive if the target date is in the future.
    pub fn distance_in_days_from(&self, target_date: NaiveDate) -> i64 {
        self.date.signed_duration_since(target_date).num_days()
    }
}

#[derive(Clone, Debug)]
pub enum SunStationKind {
    MarchEquinox,
    JuneSolstice,
    SeptemberEquinox,
    DecemberSolstice,
}

fn load_sun_stations() -> HashMap<Year, SunStationsForYear> {
    let table_string = include_str!("sun_stations.txt");
    let mut hash_map = HashMap::new();

    for line in table_string.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();

        let year = fields[0].parse::<Year>().unwrap();

        let march_equinox_date_number = fields[2].parse::<u32>().unwrap();
        let march_equinox_date =
            NaiveDate::from_ymd_opt(year, 3, march_equinox_date_number).unwrap();
        let march_equinox = SunStationWithDate {
            date: march_equinox_date,
            kind: SunStationKind::MarchEquinox,
        };

        let june_solstice_date_number = fields[5].parse::<u32>().unwrap();
        let june_solstice_date =
            NaiveDate::from_ymd_opt(year, 6, june_solstice_date_number).unwrap();
        let june_solstice = SunStationWithDate {
            date: june_solstice_date,
            kind: SunStationKind::JuneSolstice,
        };

        let september_equinox_date_number = fields[8].parse::<u32>().unwrap();
        let september_equinox_date =
            NaiveDate::from_ymd_opt(year, 9, september_equinox_date_number).unwrap();
        let september_equinox = SunStationWithDate {
            date: september_equinox_date,
            kind: SunStationKind::SeptemberEquinox,
        };

        let december_solstice_date_number = fields[11].parse::<u32>().unwrap();
        let december_solstice_date =
            NaiveDate::from_ymd_opt(year, 12, december_solstice_date_number).unwrap();
        let december_solstice = SunStationWithDate {
            date: december_solstice_date,
            kind: SunStationKind::DecemberSolstice,
        };

        let sun_stations_for_year = SunStationsForYear {
            march_equinox,
            june_solstice,
            september_equinox,
            december_solstice,
        };

        hash_map.insert(year, sun_stations_for_year);
    }

    hash_map
}
