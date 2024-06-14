// アーマードコア関連のデータ型を定義
// modelだと機体のモデルとも捉えられてしまいそうなので，Typeという名称を使用
// shere Rustクレートの作成にともない，仮の型を置いておく場所にする
// 不要になったタイミングでファイルを削除すること

import { AcAssemble } from "../../../share/assemble_type";


// ダミーデータ
export const acAssembles: AcAssemble[] = [
    {
        id: 1,
        pilotName: "V.Ⅳ Rusty",
        acName: "STEEL HAZE",
        acCardImageUrl: "/ac/steel-haze.webp",
        emblemImageUrl: "/ac/rusty.jpg",
        acImageUrls: [
            "/ac/steel-haze.webp",
            "/ac/ac.jpg",
            "/ac/ac2.jpg",
            "/ac/ac3.png",
        ],
        parts: {
            weapons: {
                rArm: "MA-E-211 SAMPU",
                lArm: "Vvc-774LS",
                rBack: "Vvc-703PM",
                lBack: "MA-J-200 RANSETSU-RF"
            },
            frame: {
                head: "NACHTREIHER/44E",
                core: "NACHTREIHER/40E",
                arms: "NACHTREIHER/46E",
                legs: "NACHTREIHER/42E",
            },
            inner: {
                booster: "NACHTREIHER/44E",
                fcs: "NACHTREIHER/40E",
                generator: "NACHTREIHER/46E",
            },
        },
        description: "アーキバスグループ強化人間部隊 ヴェスパーの第4隊長\n\nラスティはグループ傘下であるシュナイダー社の\n人材公募プログラムで見出され，半年に満たない短期でヴェスパー上位に抜擢された類を見ない経歴の持ち主である．彼は入隊以前に強化手術を受けており，詳細は不明だが本人の申告によると第8世代であるという",
        remarks: "These are remarks",
        userId: 1,
    },
    {
        id: 2,
        pilotName: "Pilot 2",
        acName: "Mech 2",
        acCardImageUrl: "/ac/mech2.jpg",
        emblemImageUrl: "/ac/emblem2.jpg",
        acImageUrls: [
            "/ac/ac.jpg",
            "/ac/ac2.jpg",
            "/ac/ac3.png",
        ],
        parts: {
            weapons: {
                rArm: "Weapon A",
                lArm: "Weapon B",
                rBack: "Weapon C",
                lBack: "Weepon D"
            },
            frame: {
                head: "Head 2",
                core: "Body 2",
                arms: "Arms 2",
                legs: "Legs 2",
            },
            inner: {
                booster: "Booster 2",
                fcs: "FCS 2",
                generator: "Generator 2",
            },
        },
        description: "This is another description",
        remarks: "These are more remarks",
        userId: 1,
    },
    // 必要に応じて他のダミーデータを追加
];