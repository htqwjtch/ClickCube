use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod clients;
mod controllers;
mod modules;
mod schemas;

#[derive(OpenApi)]
#[openapi(paths(api::cube::resolve_cube, api::click::upload_images))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/upload", axum::routing::post(api::click::upload_images))
        .route("/resolve_cube", axum::routing::get(api::cube::resolve_cube))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8013").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
