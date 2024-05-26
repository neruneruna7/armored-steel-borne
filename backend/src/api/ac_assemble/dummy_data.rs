use share::model::assemble_core::{AcAssemble, Frame, Inner, Parts, Weapons};

pub fn dumy_assembles() -> Vec<AcAssemble> {
    let ac_assemles = [
        AcAssemble {
            // ulid: Ulid::from_string("01HXVEABC7SJRCCRKBQQR1E9GF").unwrap(),
            id: 1,
            user_id: 1,
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
                inner: Inner {
                    booster: "ALULA/21E".to_string(),
                    fcs: "FCS-G2/P05".to_string(),
                    generator: "AG-T-005 HOKUSHI".to_string(),
                },
                expansion: Some("Shield".to_string()),
            },
            description: "
アーキバスグループ強化人間部隊 ヴェスパーの第4隊長

ラスティはグループ傘下であるシュナイダー社の
人材公募プログラムで見出され半年に満たない短期で
ヴェスパー上位に抜擢された類を見ない経歴の持ち主である

彼は入隊以前に強化手術を受けており 詳細は不明だが
本人の申告によると第8世代であるという"
                .to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ".to_string(),
        },
        AcAssemble {
            // ulid: Ulid::from_string("01HXVEBSR8MRC2GTBK6DVHV2XR").unwrap(),
            id: 2,
            user_id: 1,
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
                inner: Inner {
                    booster: "ALULA/21E".to_string(),
                    fcs: "FCS-G2/P05".to_string(),
                    generator: "AG-T-005 HOKUSHI".to_string(),
                },
                expansion: Some("Shield".to_string()),
            },
            description: "
ベイラムグループ専属AC部隊レッドガンの総長
ファーロン武装船団の指揮官を経てレッドガン総長となった
ミシガンは徹底して容赦のないことで知られ
「歩く地獄」 として敵味方両陣営から恐れられている

彼はベイラムのバウンティボードに自らを登録しており
死亡時懸賞金の半額は古巣ファーロンの同僚たちが
受け取る契約になっているのだという"
                .to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ2".to_string(),
        },
        AcAssemble {
            // ulid: Ulid::from_string("01HXVECA13MVR7E0DCMVZ5JGXD").unwrap(),
            id: 3,
            user_id: 1,
            pilot_name: "Honest Brute".to_string(),
            ac_name: "MILK TOOTH".to_string(),
            ac_card_image_url: "/ac/milk-tooth.jpg".to_string(),
            emblem_image_url: "/ac/rusty.jpg".to_string(),
            ac_image_urls: vec![
                "/ac/milk-tooth.jpg".to_string(),
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
                inner: Inner {
                    booster: "ALULA/21E".to_string(),
                    fcs: "FCS-G2/P05".to_string(),
                    generator: "AG-T-005 HOKUSHI".to_string(),
                },
                expansion: Some("Shield".to_string()),
            },
            description: "
ドーザー最大勢力 ジャンカーコヨーテスの頭目

元々ブルートゥはRaDに拾われた構成員であり
そのドーザーらしからぬ柔和な人当たりと話しぶりを
面白がったカーラに目をかけられていた

彼が重度の虚言癖を備えた人格破綻者であることが
発覚したとき RaDの資金と技術は無視できないレベルで
持ち逃げされていたという"
                .to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ2".to_string(),
        },
        AcAssemble {
            // ulid: Ulid::from_string("01HXVECT3C8Q3K690YXTW7AMKX").unwrap(),
            id: 4,
            user_id: 1,
            pilot_name: "G4 Volta".to_string(),
            ac_name: "CANNON HEAD".to_string(),
            ac_card_image_url: "/ac/cannon-head.jpg".to_string(),
            emblem_image_url: "/ac/rusty.jpg".to_string(),
            ac_image_urls: vec!["/ac/cannon-head.jpg".to_string()],
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
                inner: Inner {
                    booster: "ALULA/21E".to_string(),
                    fcs: "FCS-G2/P05".to_string(),
                    generator: "AG-T-005 HOKUSHI".to_string(),
                },
                expansion: Some("Shield".to_string()),
            },
            description: "
ベイラムグループ専属AC部隊「レッドガン」の4番手
総長ミシガンによる鉄拳制裁を受けたヴォルタとイグアスはその後
「青少年の健全育成」として身元を引き取られ、レッドガンで地獄の日々を送ることになる

「ミシガンの顔面に一発ぶち込んでから抜ける」という二人の目標は7年経っても達成されることなく
内心諦めた彼は今は五花海から商売を学んでいる
"
            .to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ2".to_string(),
        },
        AcAssemble {
            // ulid: Ulid::from_string("01HXVED5KARDBR3WC6Q7H00T13").unwrap(),
            id: 5,
            user_id: 1,
            pilot_name: "六文銭".to_string(),
            ac_name: "SHINOBI".to_string(),
            ac_card_image_url: "/ac/shinobi.jpg".to_string(),
            emblem_image_url: "/ac/rusty.jpg".to_string(),
            ac_image_urls: vec!["/ac/shinobi.jpg".to_string()],
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
                inner: Inner {
                    booster: "ALULA/21E".to_string(),
                    fcs: "FCS-G2/P05".to_string(),
                    generator: "AG-T-005 HOKUSHI".to_string(),
                },
                expansion: Some("Shield".to_string()),
            },
            description: "
ルビコン解放戦線に身を寄せる流浪の独立傭兵

今では失われた古典芸能に造詣が深く
中でも「ニンジャ」 「カブキ」 といった日系移民文化は
六文銭のスタイルに強い影響を与えた

彼はかつて餓死寸前のところをツィイーに救われ
そのとき施された一宿一飯の恩義に報いるため
彼女ら同志に仇なすものを敵としている        
"
            .to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ2".to_string(),
        },
        AcAssemble {
            // ulid: Ulid::from_string("01HXVEDKG7G7J4AWR98TPG4RFF").unwrap(),
            id: 6,
            user_id: 1,
            pilot_name: "六文銭".to_string(),
            ac_name: "SHINOBI".to_string(),
            ac_card_image_url: "/ac/shinobi.jpg".to_string(),
            emblem_image_url: "/ac/rusty.jpg".to_string(),
            ac_image_urls: vec!["/ac/shinobi.jpg".to_string()],
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
                inner: Inner {
                    booster: "ALULA/21E".to_string(),
                    fcs: "FCS-G2/P05".to_string(),
                    generator: "AG-T-005 HOKUSHI".to_string(),
                },
                expansion: Some("Shield".to_string()),
            },
            description: "
ルビコン解放戦線に身を寄せる流浪の独立傭兵

今では失われた古典芸能に造詣が深く
中でも「ニンジャ」 「カブキ」 といった日系移民文化は
六文銭のスタイルに強い影響を与えた

彼はかつて餓死寸前のところをツィイーに救われ
そのとき施された一宿一飯の恩義に報いるため
彼女ら同志に仇なすものを敵としている        
"
            .to_string(),
            remarks: "サーバーサイドで定義されたダミーデータ2".to_string(),
        },
    ];

    ac_assemles.to_vec()
}
