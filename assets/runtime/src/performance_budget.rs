use std::{
    fmt::Display,
    path::{Path, PathBuf},
    time::Duration,
};

// We assume 5 Mbps of bandwidth and 100 milliseconds of
// latency. These are conservative estimates based on this
// survey of U.S. mobile networks from Ookla.
//
// https://www.ookla.com/ookla-for-good/open-data
pub static assumed_latency: Duration = Duration::from_millis(100);
pub static assumed_bits_per_second: f64 = 5_000_000.0;
pub static assumed_bytes_per_second: f64 = assumed_bits_per_second / 8.0;

pub fn estimate_load_time(num_bytes: usize) -> EstimatedLoadTime {
    let num_bytes = num_bytes as f64;

    let assumed_bytes_per_milli = assumed_bytes_per_second / 1000.0;

    let num_millis = num_bytes / assumed_bytes_per_milli;
    let load_time_without_latency = Duration::from_millis(num_millis.round() as u64);

    let load_time_with_latency = load_time_without_latency + assumed_latency;

    EstimatedLoadTime {
        with_latency: load_time_with_latency,
        without_latency: load_time_without_latency,
    }
}

pub struct EstimatedLoadTime {
    with_latency: Duration,
    without_latency: Duration,
}

pub enum HowCloseToBudget {
    Below,
    AlmostOver {
        path: PathBuf,
        estimated_load_time: EstimatedLoadTime,
        budgeted_load_time: Duration,
    },
    Over {
        path: PathBuf,
        estimated_load_time: EstimatedLoadTime,
        budgeted_load_time: Duration,
    },
}

impl HowCloseToBudget {
    pub fn new<Asset: HasPerformanceBudget + ?Sized>(asset: &Asset) -> HowCloseToBudget {
        let asset_size = asset.size_in_bytes();

        let estimated_load_time = estimate_load_time(asset_size);
        let estimated_load_time_millis = estimated_load_time.with_latency.as_millis();
        let budgeted_load_time = asset.load_time_budget();
        let budgeted_load_time_millis = budgeted_load_time.as_millis();

        let one_fifth_of_budget = budgeted_load_time_millis / 5;
        // Warn if we're within 20% of the budget.
        let warning_threshold = budgeted_load_time_millis - one_fifth_of_budget;

        if (0..=warning_threshold).contains(&estimated_load_time_millis) {
            return HowCloseToBudget::Below;
        }

        let path = asset.path_for_reporting_asset_over_budget();

        if (warning_threshold..=budgeted_load_time_millis).contains(&estimated_load_time_millis) {
            return HowCloseToBudget::AlmostOver {
                path: path.to_owned(),
                estimated_load_time,
                budgeted_load_time,
            };
        }

        HowCloseToBudget::Over {
            path: path.to_owned(),
            estimated_load_time,
            budgeted_load_time,
        }
    }
}

impl Display for HowCloseToBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HowCloseToBudget::Below => Ok(()),
            HowCloseToBudget::AlmostOver {
                path,
                estimated_load_time,
                budgeted_load_time,
            } => write!(
                f,
                "{asset_path} is almost over budget.

Budgeted load time: {budgeted_load_time_millis} ms.

Esimated load time without latency: {estimated_load_time_without_latency_millis} ms.
Esimated load time with latency: {estimated_load_time_with_latency_millis} ms.

We assume a latency of {assumed_latency_millis} ms and a bandwidth of {bandwidth_mbps} Mbps, which is based on this survey of U.S. mobile networks from Ookla: https://www.ookla.com/ookla-for-good/open-data.

This means the minimum load time is always at least {assumed_latency_millis} ms.
",
                asset_path = path.to_str().unwrap(),
                estimated_load_time_without_latency_millis = estimated_load_time.without_latency.as_millis(),
                estimated_load_time_with_latency_millis = estimated_load_time.with_latency.as_millis(),
                budgeted_load_time_millis = budgeted_load_time.as_millis(),
                assumed_latency_millis = assumed_latency.as_millis(),
                bandwidth_mbps = assumed_bits_per_second / 1_000_000.0,
            ),
            HowCloseToBudget::Over {
                path,
                estimated_load_time,
                budgeted_load_time,
            } => write!(
                f,
                "{asset_path} is over budget.

Budgeted load time: {budgeted_load_time_millis} ms.

Esimated load time without latency: {estimated_load_time_without_latency_millis} ms.
Esimated load time with latency: {estimated_load_time_with_latency_millis} ms.

We assume a latency of {assumed_latency_millis} ms and a bandwidth of {bandwidth_mbps} Mbps, which is based on this survey of U.S. mobile networks from Ookla: https://www.ookla.com/ookla-for-good/open-data.

This means the minimum load time is always at least {assumed_latency_millis} ms.
",
                asset_path = path.to_str().unwrap(),
                estimated_load_time_without_latency_millis = estimated_load_time.without_latency.as_millis(),
                estimated_load_time_with_latency_millis = estimated_load_time.with_latency.as_millis(),
                budgeted_load_time_millis = budgeted_load_time.as_millis(),
                assumed_latency_millis = assumed_latency.as_millis(),
                bandwidth_mbps = assumed_bits_per_second / 1_000_000.0,
            ),
        }
    }
}

pub trait HasPerformanceBudget {
    fn check_performance_budget(&self) {
        // Only check performance budgets in release mode.
        if cfg!(debug_assertions) {
            return;
        }

        let how_close = HowCloseToBudget::new(self);
        match how_close {
            HowCloseToBudget::Below => {}
            HowCloseToBudget::AlmostOver { .. } => {
                eprintln!("{}", how_close);
            }
            HowCloseToBudget::Over { .. } => {
                panic!("{}", how_close);
            }
        }
    }

    // Used for enforcing performance budgets.
    fn load_time_budget(&self) -> Duration;

    fn size_in_bytes(&self) -> usize;

    fn path_for_reporting_asset_over_budget(&self) -> &Path;
}
