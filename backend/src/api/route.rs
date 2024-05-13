use axum::{routing::get, Json, Router};
use share::model::Assemble::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

use super::ac_assemble::handler::get_ac_asm;


pub fn route(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/ac/:ulid", get(get_ac_asm))
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
        healthz,
        super::ac_assemble::handler::get_ac_asm,
    ),
    components(schemas(
        // AcAsmGetReq,
        AcAsmGetRes,
        AcAssemble,
        Weapons,
        Frame,
        Parts,
    ))
)]
struct ApiDoc;