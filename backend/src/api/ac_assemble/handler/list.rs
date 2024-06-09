use http::StatusCode;
use share::model::assemble_reqres::{AcAsmListReq, AcAsmListRes};
use tracing::info;

use crate::api::ac_assemble::dummy_data;

use axum::{extract::Query, response::IntoResponse, Json};

#[utoipa::path(
get,
path = "/asm/list",
params(AcAsmListReq),
responses(
    (status = 200, description = "OK", body = AcAsmListRes),
),
)]
#[tracing::instrument]
pub async fn list_ac_asm(query: Option<Query<AcAsmListReq>>) -> impl IntoResponse {
    let Query(_req) = query.unwrap_or_default();

    // ここでAcAssembleのダミーデータを作成します。
    let ac_assembles = dummy_data::dumy_assembles();

    info!("ac_assembles len: {:?}", ac_assembles.len());

    (StatusCode::OK, Json(AcAsmListRes { ac_assembles }))
}
