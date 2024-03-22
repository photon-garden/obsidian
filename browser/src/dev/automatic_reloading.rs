use chrono::{DateTime, Local};
use gloo::console::log;
use gloo::history::{BrowserHistory, History};
use gloo::net::http::Request;
use gloo::timers::callback::Interval;
use once_cell::sync::OnceCell;
use shared::route::Route;
use wasm_bindgen_futures::spawn_local;

pub fn start_checking_for_updates() {
    Interval::new(1_000, || {
        spawn_local(async {
            check_for_updates().await;
        })
    })
    .forget();
}

static original_build_time_cell: OnceCell<DateTime<Local>> = OnceCell::new();

async fn check_for_updates() {
    // log!("Checking for updates!");
    if new_version_available().await {
        // Reload page.
        BrowserHistory::new().go(0);
    }
}

async fn new_version_available() -> bool {
    let maybe_most_recent_build_time = get_most_recent_build_time().await;
    match maybe_most_recent_build_time {
        Ok(most_recent_build_time) => {
            // log!("Original build time:", original_build_time.to_string());
            // log!(
            //     "Most recent server build time:",
            //     most_recent_build_time.to_string()
            // );
            let original_build_time =
                original_build_time_cell.get_or_init(|| most_recent_build_time);
            most_recent_build_time > *original_build_time
        }
        Err(error) => {
            log!(
                "Error getting most recent build time. The server might be rebuilding. Error message:",
                error.to_string()
            );
            false
        }
    }
}

async fn get_most_recent_build_time() -> Result<DateTime<Local>, gloo::net::Error> {
    let url = Route::BuildTime.to_string();
    let response = Request::get(&url).send().await?;

    if !response.ok() {
        let status_code = response.status();
        let error_message = format!("Response not ok. Status code {}.", status_code);
        let error = gloo::net::Error::GlooError(error_message);
        return Err(error);
    }

    let unparsed = response.text().await?;
    let most_recent_build_time = DateTime::parse_from_rfc3339(&unparsed)
        .expect("Couldn't parse build time from server.")
        .into();
    Ok(most_recent_build_time)
}
