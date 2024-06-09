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
    let id = repo.create(asm).await.map_err(|e| {
        info!("create_assemble error: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    // 作成したIDを返却
    let res = AcAsmPostRes { created_id: id };
    Ok(Json(res))
}

// テスト用のリクエスト
/*
curl -X 'POST' \
  'http://127.0.0.1:8000/asm/create' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
  "acAssemble": {
    "acCardImageUrl": "string",
    "acImageUrls": [
      "string"
    ],
    "acName": "string",
    "description": "string",
    "emblemImageUrl": "string",
    "parts": {
      "expansion": "Shield",
      "frame": {
        "arms": "Arms Type A",
        "core": "Core Type A",
        "head": "Head Type A",
        "legs": "Legs Type A"
      },
      "inner": {
        "booster": "Booster Type A",
        "fcs": "FCS Type A",
        "generator": "Generator Type A"
      },
      "weapons": {
        "lArm": "Laser Blade",
        "lBack": "Laser Blade",
        "rArm": "Laser Blade",
        "rBack": "Laser Blade"
      }
    },
    "pilotName": "string",
    "remarks": "string",
    "userId": 1
  }
}'
*/
