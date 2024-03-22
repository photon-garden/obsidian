use crate::extensions::*;
use axum::{extract::Request, routing::get, Router};
use routes::route::ServerSideRouteExtension;
use shared::route::Route;
use tower_http::services::ServeDir;

mod assets;
mod components;
mod css_class_groups;
mod extensions;
mod library;
mod routes;

#[tokio::main]
async fn main() {
    library_of_babel::test_leaflet();

    let built_assets_browser_prefix = {
        let browser_prefix = ::assets::paths::built_assets_browser_prefix();
        format!("/{}", browser_prefix.to_string_lossy())
    };
    let built_assets_dir = ::assets::paths::built_assets_dir();

    let app = Router::new()
        .route("/", get(handle_request)) // The wildcard "/*anthing" syntax doesn't match the root route, so we have to register that one separately.
        .route("/*anything", get(handle_request))
        .route("/healthz", get(health_check))
        .nest_service(
            &built_assets_browser_prefix,
            ServeDir::new(built_assets_dir),
        );

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host_and_port = format!("0.0.0.0:{}", port);
    // Run our app with hyper, listening globally on the specified port.
    let listener = tokio::net::TcpListener::bind(&host_and_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Listening on {}", host_and_port);
}

// For now, all of our routes return HTML.
async fn handle_request(req: Request) -> axum::response::Html<String> {
    let route = Route::from_request(&req);
    route.html().into_axum_html_response()
}

async fn health_check() {}
