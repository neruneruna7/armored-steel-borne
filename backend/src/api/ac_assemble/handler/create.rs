use axum::{extract::State, Json};
use serde::Deserialize;
use share::model::assemble_reqres::AcAsmPostReq;
use tracing::info;

use crate::AppState;

#[derive(Deserialize)]
pub struct UserRequest {
    pub email: String,
}

#[utoipa::path(
    post,
    path = "/asm/create",
    request_body = AcAsmPostReq,
    responses(
        (status = 200, description = "OK", body = String),
    ),
    )]
#[tracing::instrument(skip(state))]
pub async fn create_ac_asm(
    State(state): State<AppState>,
    Json(req): Json<AcAsmPostReq>,
) -> Result<Json<String>, ()> {
    info!("create_assemble: {:?}", req);

    Ok(Json("Ok".to_string()))
}
