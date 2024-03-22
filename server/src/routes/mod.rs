use crate::assets::processed_2023_haiku;
use crate::components::{Layout, Link};
use crate::extensions::VecExtension;
use crate::library::seasons::{get_closest_upcoming_solstice_or_equinox, SunStationKind};
use maud::{html, Markup, Render};
use number_to_words::number_to_words;
use shared::route::Route;

mod build_time;
mod not_found;
pub mod route;
mod santoka;
mod work;

pub fn page() -> Markup {
    Layout::new(
        "Luca Aurelia",
        "Luca Aurelia",
        html! {
            main
                class="p-8 flex flex-col items-center justify-between h-dvh" {
                (spacer())
                (centerpiece())
                (desktop_nav())
            }
        },
    )
    .render()
}

fn spacer() -> Markup {
    html! {
        div class="spacer w-full h-12" {}
    }
}

fn centerpiece() -> Markup {
    html! {
        div class="centerpiece flex flex-row items-center" {
            (poem())
            (glass())
        }
    }
}

fn poem() -> Markup {
    let today = chrono::Local::now().date_naive();
    let closest_poems = processed_2023_haiku::find_closest_poems(&today);
    let poem = closest_poems.random_element();
    let closest_sun_station = get_closest_upcoming_solstice_or_equinox();
    let days_until_next_sun_station = closest_sun_station.distance_in_days_from(today);

    let num_days = number_to_words(days_until_next_sun_station as i32, false);
    let name_of_sun_station = match closest_sun_station.kind {
        SunStationKind::MarchEquinox => "spring (or fall) equinox",
        SunStationKind::JuneSolstice => "summer (or winter) solstice",
        SunStationKind::SeptemberEquinox => "spring (or fall) equinox",
        SunStationKind::DecemberSolstice => "summer (or winter) solstice",
    };

    html! {
        // We force the .poem-text to line-wrap to the width
        // of the .sun-station by:
        // - Setting whitespace-nowrap on the .sun-station paragraph.
        // - Setting .w-min on the .poem div, shrinking it to the width
        //   of .sun-station.
        // - Setting .w-full on the .poem-text paragraph.
        div class="poem w-min text-4xl flex flex-col gap-2" {
            p class="sun-station whitespace-nowrap font-extralight text-neutral-500 dark:text-neutral-300" {
                @if days_until_next_sun_station == 0 {
                    "today is the " (name_of_sun_station)
                } @else {
                    (num_days) " days to the " (name_of_sun_station)
                }
            }

            p class="poem-text w-full font-thin italic text-neutral-400 dark:text-neutral-400" {
                (poem.text)
            }
        }
    }
}

fn glass() -> Markup {
    html! {
        div
            class={"rounded-full -ml-32 w-80 h-80 shadow-[0_4px_4px_rgb(10_10_10_/_0.25)] dark:shadow-[0_4px_4px_rgb(10_10_10_/_0.5)] shrink-0 backdrop-blur-[2px] bg-gradient-to-br from-cerulean to-cloud dark:from-twilight dark:to-neutral-900/5" } {
        }
    }
}

fn desktop_nav() -> Markup {
    html! {
        nav class="desktop-nav flex flex-row h-12 text-base font-light text-neutral-400" {
            ul class="flex flex-row gap-4 h-full items-center" {
                li {
                    "Hi, my name is Luca Aurelia"
                }
                li {
                    (Link::new()
                        .href(Route::Work)
                        .slot("work")
                    )
                }
                li {
                    (Link::new()
                        .href(Route::Santoka)
                        .slot("santōka")
                    )
                }
            }
        }
    }
}
