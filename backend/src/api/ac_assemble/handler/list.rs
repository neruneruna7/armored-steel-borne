use http::StatusCode;
use share::model::Assemble::{AcAsmListReq, AcAsmListRes, AcAssemble, Frame, Parts, Weapons};
use ulid::Ulid;

use crate::api::ac_assemble::dummy_data;

use super::super::model::{AcAsmGetReq, AcAsmGetRes};
use axum::{extract::Query, response::IntoResponse, Json};

#[utoipa::path(
get,
path = "/asm",
params(AcAsmListReq),
responses(
    (status = 200, description = "OK", body = AcAsmListRes),
),
)]
pub async fn list_ac_asm(query: Option<Query<AcAsmListReq>>) -> impl IntoResponse {
    let Query(_req) = query.unwrap_or_default();

    // ここでAcAssembleのダミーデータを作成します。
    let ac_assembles = dummy_data::dumy_assembles();

    (
        StatusCode::OK,
        Json(AcAsmListRes {
            ac_assembles: ac_assembles,
        }),
    )
}
