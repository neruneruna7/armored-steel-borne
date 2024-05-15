use axum::{routing::get, Json, Router};
use share::model::{assemble_core::*, assemble_reqres::*};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

use super::ac_assemble::handler::{get_ac_asm, list_ac_asm};

pub fn route(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/asm/:ulid", get(get_ac_asm))
        .route("/asm/list", get(list_ac_asm))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
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
        super::ac_assemble::handler::list_ac_asm,
    ),
    components(schemas(
        // AcAsmGetReq,
        AcAsmGetRes,
        AcAsmListRes,
        AcAssemble,
        Weapons,
        Frame,
        Parts,
    ))
)]
struct ApiDoc;
