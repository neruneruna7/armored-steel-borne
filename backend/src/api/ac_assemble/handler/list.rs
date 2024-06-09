use http::StatusCode;
use share::model::assemble_reqres::{AcAsmListReq, AcAsmListRes};
use tracing::info;

use crate::{api::ac_assemble::dummy_data, repository::assemblies::Ac6AssembliesRepo, AppState};

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};

#[utoipa::path(
get,
path = "/asm/list",
params(AcAsmListReq),
responses(
    (status = 200, description = "OK", body = AcAsmListRes),
),
)]
#[tracing::instrument(skip(state))]
pub async fn list_ac_asm(
    query: Option<Query<AcAsmListReq>>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let Query(req) = query.unwrap_or_default();
    // ここでAcAssembleのダミーデータを作成します。
    // let ac_assembles = dummy_data::dumy_assembles();

    // DBからAcAssembleを取得
    let repo = Ac6AssembliesRepo::new(state.postgres.clone());
    let ac_assembles = repo
        .read_list(req.prev_id.unwrap_or_default(), req.size.unwrap())
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    info!("ac_assembles len: {:?}", ac_assembles.len());

    Ok((StatusCode::OK, Json(AcAsmListRes { ac_assembles })))
}
