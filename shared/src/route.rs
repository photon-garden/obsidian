use super::santoka_haiku_2023_06_26::PublicationId;
use enum_iterator;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Copy, enum_iterator::Sequence, Serialize, Deserialize, Debug)]
pub enum Route {
    BuildTime,
    Home,
    NotFound,
    NonPreviewPoems { publication_id: PublicationId },
    Santoka,
    Work,
}

impl Route {
    pub fn all() -> impl Iterator<Item = Route> {
        enum_iterator::all::<Route>()
    }

    pub fn parse_path(path: &str) -> Route {
        Route::all()
            .find(|route| route.to_string() == path)
            .unwrap_or(Route::NotFound)
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let route_str = match self {
            Route::BuildTime => "/build-time".to_string(),
            Route::Home => "/".to_string(),
            Route::NonPreviewPoems { publication_id } => {
                format!("/non-preview-poems/{}", publication_id)
            }
            Route::NotFound => "/not-found".to_string(),
            Route::Santoka => "/santoka".to_string(),
            Route::Work => "/work".to_string(),
        };

        write!(f, "{}", route_str)
    }
}
