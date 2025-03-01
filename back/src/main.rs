use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod clients;
mod controllers;
mod models;
mod modules;

#[derive(OpenApi)]
#[openapi(paths(
    api::click::upload_images,
    api::click::detect_colors,
    api::cube::resolve_cube,
))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/upload_images",
            axum::routing::post(api::click::upload_images),
        )
        .route(
            "/detect_colors",
            axum::routing::get(api::click::detect_colors),
        )
        .route("/resolve_cube", axum::routing::get(api::cube::resolve_cube))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
