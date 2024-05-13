use http::StatusCode;
use share::model::Assemble::{AcAsmListReq, AcAsmListRes, AcAssemble, Frame, Parts, Weapons};
use ulid::Ulid;

use super::super::model::{AcAsmGetReq, AcAsmGetRes};
use axum::{extract::Query, response::IntoResponse, Json};

#[utoipa::path(
get,
path = "/ac",
params(AcAsmListReq),
responses(
    (status = 200, description = "OK", body = AcAsmListRes),
),
)]
pub async fn list_ac_asm(query: Option<Query<AcAsmListReq>>) -> impl IntoResponse {
    let Query(req) = query.unwrap_or_default();

    // ここでAcAssembleのダミーデータを作成します。
    let ulid = Ulid::new();
    let ac_assembles = [
        AcAssemble {
            ulid: ulid,
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
        },
        AcAssemble {
            ulid: ulid,
            pilot_name: "G1 Michigan".to_string(),
            ac_name: "LIGER TAIL".to_string(),
            ac_card_image_url: "/ac/liger-tail.jpg".to_string(),
            emblem_image_url: "/ac/rusty.jpg".to_string(),
            ac_image_urls: vec![
                "/ac/liger-tail.jpg".to_string(),
                "/ac/liger-tail-2.jpg".to_string(),
                "/ac/liger-tail-3.jpg".to_string(),
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
            description: "
ベイラムグループ専属AC部隊レッドガンの総長
ファーロン武装船団の指揮官を経てレッドガン総長となった
ミシガンは徹底して容赦のないことで知られ
「歩く地獄」 として敵味方両陣営から恐れられている

彼はベイラムのバウンティボードに自らを登録しており
死亡時懸賞金の半額は古巣ファーロンの同僚たちが
受け取る契約になっているのだという".to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ2".to_string(),
        },
        AcAssemble {
            ulid: ulid,
            pilot_name: "Honest Brute".to_string(),
            ac_name: "LIGER TAIL".to_string(),
            ac_card_image_url: "/ac/milk-tooth.jpg".to_string(),
            emblem_image_url: "/ac/rusty.jpg".to_string(),
            ac_image_urls: vec![
                "/ac/milk-tooth".to_string(),
                "/ac/milk-tooth-2.jpg".to_string(),
                "/ac/milk-tooth-3.jpg".to_string(),
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
            description: "
ドーザー最大勢力 ジャンカーコヨーテスの頭目

元々ブルートゥはRaDに拾われた構成員であり
そのドーザーらしからぬ柔和な人当たりと話しぶりを
面白がったカーラに目をかけられていた

彼が重度の虚言癖を備えた人格破綻者であることが
発覚したとき RaDの資金と技術は無視できないレベルで
持ち逃げされていたという".to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ2".to_string(),
        },
        AcAssemble {
            ulid: ulid,
            pilot_name: "G4 Volta".to_string(),
            ac_name: "CANNON HEAD".to_string(),
            ac_card_image_url: "/ac/cannon-head.jpg".to_string(),
            emblem_image_url: "/ac/rusty.jpg".to_string(),
            ac_image_urls: vec![
                "/ac/cannon-head".to_string(),
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
            description: "
ベイラムグループ専属AC部隊「レッドガン」の4番手
総長ミシガンによる鉄拳制裁を受けたヴォルタとイグアスはその後
「青少年の健全育成」として身元を引き取られ、レッドガンで地獄の日々を送ることになる

「ミシガンの顔面に一発ぶち込んでから抜ける」という二人の目標は7年経っても達成されることなく
内心諦めた彼は今は五花海から商売を学んでいる
".to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ2".to_string(),
        },
        AcAssemble {
            ulid: ulid,
            pilot_name: "六文銭".to_string(),
            ac_name: "SHINOBI".to_string(),
            ac_card_image_url: "/ac/shinobi.jpg".to_string(),
            emblem_image_url: "/ac/rusty.jpg".to_string(),
            ac_image_urls: vec![
                "/ac/shinobi".to_string(),
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
            description: "
ルビコン解放戦線に身を寄せる流浪の独立傭兵

今では失われた古典芸能に造詣が深く
中でも「ニンジャ」 「カブキ」 といった日系移民文化は
六文銭のスタイルに強い影響を与えた

彼はかつて餓死寸前のところをツィイーに救われ
そのとき施された一宿一飯の恩義に報いるため
彼女ら同志に仇なすものを敵としている        
".to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ2".to_string(),
        },
        AcAssemble {
            ulid: ulid,
            pilot_name: "六文銭".to_string(),
            ac_name: "SHINOBI".to_string(),
            ac_card_image_url: "/ac/shinobi.jpg".to_string(),
            emblem_image_url: "/ac/rusty.jpg".to_string(),
            ac_image_urls: vec![
                "/ac/shinobi".to_string(),
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
            description: "
ルビコン解放戦線に身を寄せる流浪の独立傭兵

今では失われた古典芸能に造詣が深く
中でも「ニンジャ」 「カブキ」 といった日系移民文化は
六文銭のスタイルに強い影響を与えた

彼はかつて餓死寸前のところをツィイーに救われ
そのとき施された一宿一飯の恩義に報いるため
彼女ら同志に仇なすものを敵としている        
".to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ2".to_string(),
        },
    ];
    (
        StatusCode::OK,
        Json(AcAsmListRes {
            ac_assembles: ac_assembles.to_vec(),
        }),
    )
}