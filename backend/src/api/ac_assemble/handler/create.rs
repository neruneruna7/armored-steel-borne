use axum::{extract::State, Json};
use http::StatusCode;
use serde::Deserialize;
use share::model::assemble_reqres::{AcAsmPostReq, AcAsmPostRes};
use tracing::info;

use crate::{repository::assemblies::Ac6AssembliesRepo, AppState};

#[derive(Deserialize)]
pub struct UserRequest {
    pub email: String,
}

#[utoipa::path(
    post,
    path = "/asm/create",
    request_body = AcAsmPostReq,
    responses(
        (status = 200, description = "OK", body = AcAsmPostRes),
    ),
    )]
#[tracing::instrument(skip(state))]
pub async fn create_ac_asm(
    State(state): State<AppState>,
    Json(req): Json<AcAsmPostReq>,
) -> Result<Json<AcAsmPostRes>, StatusCode> {
    info!("create_assemble: {:?}", req);

    // reqをAssembleNonIDとUserIDに分解
    let asm = req.ac_assemble;

    // リクエストをもとにDBへクエリ発行
    let repo = Ac6AssembliesRepo::new(state.postgres.clone());
    let id = repo
        .create(asm)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 作成したIDを返却
    let res = AcAsmPostRes { created_id: id };
    Ok(Json(res))
}
