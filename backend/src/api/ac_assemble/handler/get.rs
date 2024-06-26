use http::StatusCode;
use share::model::assemble_reqres::{AcAsmGetReq, AcAsmGetRes};
use tracing::info;

use crate::api::ac_assemble::dummy_data;

use axum::{extract::Path, response::IntoResponse, Json};

#[utoipa::path(
get,
path = "/asm/{ulid}",
context_path = "api",
// params(Ulid),
responses(
    (status = 200, description = "OK", body = AcAsmGetRes),
),
)]
#[tracing::instrument]
pub async fn get_ac_asm(Path(req): Path<AcAsmGetReq>) -> impl IntoResponse {
    // ここでAcAssembleのダミーデータを作成します。
    let ac_assemble = dummy_data::dumy_assembles();

    // reqのUlidと一致するAcAssembleを取得
    // ひとまずエラーハンドリングはせずに動作確認
    let ac_assemble = ac_assemble
        .into_iter()
        .find(|ac_assemble| ac_assemble.id == req)
        .unwrap();

    info!("ac_assemble name: {:?}", ac_assemble.ac_name);

    (StatusCode::OK, Json(AcAsmGetRes { ac_assemble }))
}
