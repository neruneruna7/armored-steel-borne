use http::StatusCode;
use share::model::assemble_reqres::{AcAsmGetReq, AcAsmGetRes};
use tracing::info;

use crate::{api::ac_assemble::dummy_data, repository::assemblies::Ac6AssembliesRepo, AppState};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

#[utoipa::path(
get,
path = "/asm/{id}",
context_path = "api",
params(AcAsmGetReq),
responses(
    (status = 200, description = "OK", body = AcAsmGetRes),
),
)]
#[tracing::instrument(skip(state))]
pub async fn get_ac_asm(
    Path(req): Path<AcAsmGetReq>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    // // ここでAcAssembleのダミーデータを作成します。
    // let ac_assemble = dummy_data::dumy_assembles();

    // // reqのUlidと一致するAcAssembleを取得
    // // ひとまずエラーハンドリングはせずに動作確認
    // let ac_assemble = ac_assemble
    //     .into_iter()
    //     .find(|ac_assemble| ac_assemble.id == req)
    //     .unwrap();

    // reqのidをもとに，DBからAcAssembleを取得
    let repo = Ac6AssembliesRepo::new(state.postgres.clone());
    let ac_assemble = repo
        .read(req.id)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    info!("ac_assemble name: {:?}", ac_assemble.ac_name);

    Ok((StatusCode::OK, Json(AcAsmGetRes { ac_assemble })))
}
