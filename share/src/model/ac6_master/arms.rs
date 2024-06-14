/// 腕パーツがもつパラメータなど
#[derive(Debug, Clone)]
pub struct Arm {
    _name: String,
    _info: String,
}

/// 腕パーツ一覧
// ハードコードしようかとも思ったが，やめよう
// 必要に応じてDBから取得する処理を入れればいい
#[derive(Debug, Clone)]
pub enum ArmList {
    ArmsTypeA(Arm),
    ArmsTypeB(Arm),
}
