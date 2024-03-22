#![allow(non_upper_case_globals)]

use date::*;
use obsidian::*;

mod date;
pub mod extensions;
mod leaflet;
mod obsidian;
mod years;

// pub fn main() {
//     years::year_2023::convert_2023_haiku_to_csv();
// let vault_path = "/Users/photon-garden/library-of-babel";
// let vault_path = "/Users/photon-garden/obsidian-dev";

// let mut vault = Vault::load_from_disk(vault_path);
// move_people_into_people_folder(&mut vault);
// create_dates_for_year(&mut vault, 2024);
// }

pub use years::year_2023::convert_2023_haiku_to_csv;
pub use years::year_2024::test_leaflet;

pub fn move_people_into_people_folder(vault: &mut Vault) {
    let people_topic_id = vault
        .item_at_path("topics + tags/People.md")
        .expect("Couldn't find the people topic in the vault.")
        .try_into_page()
        .expect("People topic wasn't a page.")
        .id
        .clone();

    let people_not_in_the_people_folder = vault
        .pages_mut()
        .filter(|page| page.has_a_reference_to(&people_topic_id))
        .filter(|page| !page.file.path_from_vault_root.starts_with("people"));

    for person in people_not_in_the_people_folder {
        let path_from_vault_root = &person.file.path_from_vault_root;
        let new_path = format!("people/{path_from_vault_root}");
        person.file.move_file(&new_path);
    }
}

pub fn create_dates_for_year(vault: &mut Vault, year: i32) {
    let year = Year::new(year);

    let year_id = get_id_for_unit_of_time(UnitOfTime::Year(year));
    vault.find_or_create_page(year_id, || {
        "[[Years]]\n\nTheme:\n\nImportant events\n- ".to_string()
    });

    for month in date::months() {
        let month_id = get_id_for_unit_of_time(UnitOfTime::Month(year, month));
        vault.find_or_create_page(month_id, || format!("[[{year}]], [[Months]]"));

        for day_of_the_month in date::days_in_month(year, month) {
            let day_id = get_id_for_unit_of_time(UnitOfTime::Day(year, month, day_of_the_month));
            vault.find_or_create_page(day_id, || {
                format!("[[{year}]], [[{year}.{month}]], [[Days]]")
            });
        }
    }
}

fn get_id_for_unit_of_time(unit_of_time: UnitOfTime) -> VaultItemId {
    let file_name = match unit_of_time {
        UnitOfTime::Year(year) => {
            format!("{year}.md")
        }

        UnitOfTime::Month(year, month) => {
            format!("{year}.{month}.md")
        }

        UnitOfTime::Day(year, month, day) => {
            format!("{year}.{month}.{day}.md")
        }
    };

    let year = unit_of_time.year();
    let path = format!("{year}/{file_name}");

    let path = if unit_of_time.in_current_year() {
        path
    } else {
        format!("years/{path}")
    };

    VaultItemId::from_path_from_vault_root(path)
}

enum UnitOfTime {
    Year(Year),
    Month(Year, date::Month),
    Day(Year, date::Month, DayOfTheMonth),
}

impl UnitOfTime {
    fn year(&self) -> &Year {
        match self {
            UnitOfTime::Year(year) => year,
            UnitOfTime::Month(year, ..) => year,
            UnitOfTime::Day(year, ..) => year,
        }
    }

    fn in_current_year(&self) -> bool {
        self.year() == &date::current_year()
    }
}
