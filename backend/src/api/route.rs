use axum::{routing::get, Json, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;


pub fn route(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .merge(
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .with_state(state)
}

#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = 200, description = "OK", body = &'static str),
    )
)]
async fn healthz() -> Json<&'static str> {
    Json("Ok")
}

#[derive(OpenApi)]
#[openapi(
    paths(
        healthz
    )
)]
struct ApiDoc;