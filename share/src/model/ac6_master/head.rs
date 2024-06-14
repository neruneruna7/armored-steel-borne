/// パーツがもつパラメータなど
#[derive(Debug, Clone)]
pub struct Head {
    _name: String,
    _info: String,
}

/// パーツ一覧
// ハードコードしようかとも思ったが，やめよう
// 必要に応じてDBから取得する処理を入れればいい
#[derive(Debug, Clone)]
pub enum HeadList {
    HeadTypeA(Head),
    HeadTypeB(Head),
}
