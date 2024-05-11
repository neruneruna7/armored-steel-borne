// アーマードコア関連のデータ型を定義
// modelだと機体のモデルとも捉えられてしまいそうなので，Typeという名称を使用

interface AcAssemble {
    pilotName: string,
    acName: string;
    acImageUrl: string;
    emblemImageUrl: string;
}

interface Parts {
    weapons: Weapons;
}

interface Weapons {
    rArm: string; //腕
    lArm: string;
    rBack: string; // 肩
}

interface frame {
    head: string,
    core: string,
    
}