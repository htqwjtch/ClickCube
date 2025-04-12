use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::env;

mod api;
mod clients;
mod controllers;
mod modules;
mod schemas;

#[derive(OpenApi)]
#[openapi(paths(
    api::click::upload_images,
    api::click::detect_colors,
    api::painter::update_colors,
    api::cube::solve,
))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow requests from any domains
        .allow_methods(Any) // Allow any methods (POST, GET, etc.)
        .allow_headers(Any); // Allow any headers

    let port = env::var("PORT").unwrap_or_else(|_| "8013".to_string());
    let address = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route(
            "/upload-images",
            axum::routing::post(api::click::upload_images),
        )
        .route(
            "/detect-colors",
            axum::routing::get(api::click::detect_colors),
        )
        .route(
            "/update-colors",
            axum::routing::post(api::painter::update_colors),
        )
        .route("/solve", axum::routing::get(api::cube::solve))
        .layer(cors)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
