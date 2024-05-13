use http::StatusCode;
use share::model::Assemble::{AcAssemble, Frame, Parts, Weapons};
use ulid::Ulid;

use super::super::model::{AcAsmGetReq, AcAsmGetRes};
use axum::{extract::{Path, Query}, response::IntoResponse, Json};

#[utoipa::path(
get,
path = "/ac/{ulid}",
context_path = "api",
responses(
    (status = 200, description = "OK", body = AcAsmGetRes),
),
)]
pub async fn get_ac_asm(Path(req): Path<AcAsmGetReq>) -> impl IntoResponse {
// ここでAcAssembleのダミーデータを作成します。
let ac_assemble = AcAssemble {
    ulid: req,
    pilot_name: "V.Ⅳ Rusty".to_string(),
    ac_name: "STEEL HAZE".to_string(),
    ac_card_image_url: "/ac/steel-haze.webp".to_string(),
    emblem_image_url: "/ac/rusty.jpg".to_string(),
    ac_image_urls: vec![
        "/ac/steel-haze.webp".to_string(),
        "/ac/ac.jpg".to_string(),
        "/ac/ac2.jpg".to_string(),
        "/ac/ac3.png".to_string(),
    ],
    parts: Parts {
        weapons: Weapons {
            r_arm: "MA-E-211 SAMPU".to_string(),
            l_arm: "Vvc-774LS".to_string(),
            r_back: "Vvc-703PM".to_string(),
            l_back: "MA-J-200 RANSETSU-RF".to_string(),
        },
        frame: Frame {
            head: "NACHTREIHER/44E".to_string(),
            core: "NACHTREIHER/40E".to_string(),
            arms: "NACHTREIHER/46E".to_string(),
            legs: "NACHTREIHER/42E".to_string(),
        },
    },
    description: "アーキバスグループ強化人間部隊 ヴェスパーの第4隊長\n\nラスティはグループ傘下であるシュナイダー社の\n人材公募プログラムで見出され，半年に満たない短期でヴェスパー上位に抜擢された類を見ない経歴の持ち主である．彼は入隊以前に強化手術を受けており，詳細は不明だが本人の申告によると第8世代であるという".to_string(),
    remarks: "サーバーサイドで定義されたダミーデータ".to_string(),
};
(
    StatusCode::OK,
    Json(
        AcAsmGetRes {
            ac_assemble
        }
    )
)
}
