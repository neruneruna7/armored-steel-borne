use http::StatusCode;
use share::model::Assemble::{AcAssemble, Frame, Parts, Weapons};
use ulid::Ulid;

use crate::api::ac_assemble::dummy_data;

use super::super::model::{AcAsmGetReq, AcAsmGetRes};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};

#[utoipa::path(
get,
path = "/asm/{ulid}",
context_path = "api",
// params(Ulid),
responses(
    (status = 200, description = "OK", body = AcAsmGetRes),
),
)]
pub async fn get_ac_asm(Path(req): Path<AcAsmGetReq>) -> impl IntoResponse {
    // ここでAcAssembleのダミーデータを作成します。
    let ac_assemble = dummy_data::dumy_assembles();

    // reqのUlidと一致するAcAssembleを取得
    // ひとまずエラーハンドリングはせずに動作確認
    let ac_assemble = ac_assemble
        .into_iter()
        .find(|ac_assemble| ac_assemble.ulid == req)
        .unwrap();

    (StatusCode::OK, Json(AcAsmGetRes { ac_assemble }))
}
