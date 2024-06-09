use anyhow::Result;
use serde::Deserialize;
use share::model::assemble_core::{AcAssemble, AcAssembleNonId, Frame, Inner, Parts, Weapons};
use sqlx::types::Json;
use sqlx::FromRow;
use sqlx::PgPool;

#[derive(Debug, Deserialize, FromRow)]
struct Ac6AssemblyRead {
    id: i32,
    pilot_name: String,
    ac_name: String,
    description: String,
    remarks: String,
    ac_card_image_url: String,
    emblem_image_url: String,
    ac_image_urls: Json<Vec<String>>,
    l_arm_name: String,
    r_arm_name: String,
    l_back_name: String,
    r_back_name: String,
    head_name: String,
    core_name: String,
    arms_name: String,
    legs_name: String,
    booster_name: String,
    fcs_name: String,
    generator_name: String,
    expansion_name: Option<String>,
    user_id: i32,
}

impl From<Ac6AssemblyRead> for AcAssemble {
    fn from(val: Ac6AssemblyRead) -> Self {
        AcAssemble {
            id: val.id,
            user_id: val.user_id,
            pilot_name: val.pilot_name,
            ac_name: val.ac_name,
            remarks: val.remarks,
            description: val.description,
            ac_card_image_url: val.ac_card_image_url,
            emblem_image_url: val.emblem_image_url,
            ac_image_urls: val.ac_image_urls.to_vec(),
            parts: Parts {
                weapons: Weapons {
                    l_arm: val.l_arm_name,
                    r_arm: val.r_arm_name,
                    l_back: val.l_back_name,
                    r_back: val.r_back_name,
                },
                frame: Frame {
                    head: val.head_name,
                    core: val.core_name,
                    arms: val.arms_name,
                    legs: val.legs_name,
                },
                inner: Inner {
                    booster: val.booster_name,
                    fcs: val.fcs_name,
                    generator: val.generator_name,
                },
                expansion: val.expansion_name,
            },
        }
    }
}

impl From<Ac6AssemblyRead> for AcAssembleNonId {
    fn from(val: Ac6AssemblyRead) -> Self {
        AcAssembleNonId {
            user_id: val.user_id,
            pilot_name: val.pilot_name,
            ac_name: val.ac_name,
            remarks: val.remarks,
            description: val.description,
            ac_card_image_url: val.ac_card_image_url,
            emblem_image_url: val.emblem_image_url,
            ac_image_urls: val.ac_image_urls.to_vec(),
            parts: Parts {
                weapons: Weapons {
                    l_arm: val.l_arm_name,
                    r_arm: val.r_arm_name,
                    l_back: val.l_back_name,
                    r_back: val.r_back_name,
                },
                frame: Frame {
                    head: val.head_name,
                    core: val.core_name,
                    arms: val.arms_name,
                    legs: val.legs_name,
                },
                inner: Inner {
                    booster: val.booster_name,
                    fcs: val.fcs_name,
                    generator: val.generator_name,
                },
                expansion: val.expansion_name,
            },
        }
    }
}

#[derive(Debug, Deserialize, FromRow)]
struct Ac6AssemblyInsert {
    pilot_name: String,
    ac_name: String,
    description: String,
    remarks: String,
    ac_card_image_url: String,
    emblem_image_url: String,
    ac_image_urls: Vec<String>,
    l_arm_name: String,
    r_arm_name: String,
    l_back_name: String,
    r_back_name: String,
    head_name: String,
    core_name: String,
    arms_name: String,
    legs_name: String,
    booster_name: String,
    fcs_name: String,
    generator_name: String,
    expansion_name: Option<String>,
    user_id: i32,
}

impl Ac6AssemblyInsert {
    fn new(
        pilot_name: String,
        ac_name: String,
        description: String,
        remarks: String,
        ac_card_image_url: String,
        emblem_image_url: String,
        ac_image_urls: Vec<String>,
        l_arm_name: String,
        r_arm_name: String,
        l_back_name: String,
        r_back_name: String,
        head_name: String,
        core_name: String,
        arms_name: String,
        legs_name: String,
        booster_name: String,
        fcs_name: String,
        generator_name: String,
        expansion_name: Option<String>,
        user_id: i32,
    ) -> Self {
        Self {
            pilot_name,
            ac_name,
            description,
            remarks,
            ac_card_image_url,
            emblem_image_url,
            ac_image_urls,
            l_arm_name,
            r_arm_name,
            l_back_name,
            r_back_name,
            head_name,
            core_name,
            arms_name,
            legs_name,
            booster_name,
            fcs_name,
            generator_name,
            expansion_name,
            user_id,
        }
    }

    // インナーパーツはまだ未実装
    fn from_acasm_nonid(value: AcAssembleNonId, user_id: i32) -> Self {
        Ac6AssemblyInsert::new(
            value.pilot_name,
            value.ac_name,
            value.description,
            value.remarks,
            value.ac_card_image_url,
            value.emblem_image_url,
            value.ac_image_urls,
            value.parts.weapons.l_arm,
            value.parts.weapons.r_arm,
            value.parts.weapons.l_back,
            value.parts.weapons.r_back,
            value.parts.frame.head,
            value.parts.frame.core,
            value.parts.frame.arms,
            value.parts.frame.legs,
            value.parts.inner.booster,
            value.parts.inner.fcs,
            value.parts.inner.generator,
            value.parts.expansion,
            user_id,
        )
    }
}

pub struct Ac6AssembliesRepo {
    db: PgPool,
}

/// 新しくレコードを挿入する際，RETURNINGでidを取得するための構造体
/// query_as!マクロが構造体しかとらない仕様のため，こうやって構造体を定義している
#[allow(dead_code)]
struct ReturnCreate {
    id: i32,
}

impl Ac6AssembliesRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, asm: AcAssembleNonId, user_id: i32) -> Result<i32> {
        let asm = Ac6AssemblyInsert::from_acasm_nonid(asm, user_id);
        let r = sqlx::query_as!(
            ReturnCreate,
            r#"
            INSERT INTO ac6_assemblies (
                pilot_name,
                ac_name,
                description,
                remarks,
                ac_card_image_url,
                emblem_image_url,
                ac_image_urls,
                l_arm_name,
                r_arm_name,
                l_back_name,
                r_back_name,
                head_name,
                core_name,
                arms_name,
                legs_name,
                booster_name,
                fcs_name,
                generator_name,
                expansion_name,
                user_id
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20
            )
            RETURNING id
            "#,
            asm.pilot_name,
            asm.ac_name,
            asm.description,
            asm.remarks,
            asm.ac_card_image_url,
            asm.emblem_image_url,
            &asm.ac_image_urls,
            asm.l_arm_name,
            asm.r_arm_name,
            asm.l_back_name,
            asm.r_back_name,
            asm.head_name,
            asm.core_name,
            asm.arms_name,
            asm.legs_name,
            asm.booster_name,
            asm.fcs_name,
            asm.generator_name,
            asm.expansion_name,
            asm.user_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(r.id)
    }

    pub async fn read(&self, id: i32) -> Result<AcAssemble> {
        let asm = sqlx::query_as!(
            Ac6AssemblyRead,
            r#"
            SELECT
                id,
                pilot_name,
                ac_name,
                description,
                remarks,
                ac_card_image_url,
                emblem_image_url,
                ac_image_urls,
                l_arm_name,
                r_arm_name,
                l_back_name,
                r_back_name,
                head_name,
                core_name,
                arms_name,
                legs_name,
                booster_name,
                fcs_name,
                generator_name,
                expansion_name,
                user_id
            FROM ac6_assemblies
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(asm.into())
    }

    // 複数のレコードを取得する
    pub async fn read_list(&self, prev_id: i32, limit_size: i64) -> Result<Vec<AcAssemble>> {
        let asm = sqlx::query_as!(
            Ac6AssemblyRead,
            r#"
            SELECT
                id,
                pilot_name,
                ac_name,
                description,
                remarks,
                ac_card_image_url,
                emblem_image_url,
                ac_image_urls,
                l_arm_name,
                r_arm_name,
                l_back_name,
                r_back_name,
                head_name,
                core_name,
                arms_name,
                legs_name,
                booster_name,
                fcs_name,
                generator_name,
                expansion_name,
                user_id
            FROM ac6_assemblies
            WHERE id >= $1
            ORDER BY id DESC
            LIMIT $2
            "#,
            prev_id,
            limit_size
        )
        .fetch_all(&self.db)
        .await?;
        Ok(asm.into_iter().map(|a| a.into()).collect())
    }

    pub async fn update(&self, asm: AcAssemble) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE ac6_assemblies
            SET
                pilot_name = $1,
                ac_name = $2,
                description = $3,
                remarks = $4,
                ac_card_image_url = $5,
                emblem_image_url = $6,
                ac_image_urls = $7,
                l_arm_name = $8,
                r_arm_name = $9,
                l_back_name = $10,
                r_back_name = $11,
                head_name = $12,
                core_name = $13,
                arms_name = $14,
                legs_name = $15,
                booster_name = $16,
                fcs_name = $17,
                generator_name = $18,
                expansion_name = $19,
                user_id = $20
            WHERE id = $21
            "#,
            asm.pilot_name,
            asm.ac_name,
            asm.description,
            asm.remarks,
            asm.ac_card_image_url,
            asm.emblem_image_url,
            &asm.ac_image_urls,
            asm.parts.weapons.l_arm,
            asm.parts.weapons.r_arm,
            asm.parts.weapons.l_back,
            asm.parts.weapons.r_back,
            asm.parts.frame.head,
            asm.parts.frame.core,
            asm.parts.frame.arms,
            asm.parts.frame.legs,
            asm.parts.inner.booster,
            asm.parts.inner.fcs,
            asm.parts.inner.generator,
            asm.parts.expansion,
            asm.user_id,
            asm.id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM ac6_assemblies
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use share::model::assemble_core::AcAssembleNonId;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    #[tokio::test]
    /// テストデータを挿入する
    async fn test_create() {
        dotenv().ok();

        let db = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();
        let repo = Ac6AssembliesRepo::new(db);

        let asm = AcAssembleNonId {
            user_id: 1,
            pilot_name: "test pilot".to_owned(),
            ac_name: "test ac ".to_owned(),
            description: "test description".to_owned(),
            remarks: "test remark".to_owned(),
            ac_card_image_url: "test_url".to_owned(),
            emblem_image_url: "test_url".to_owned(),
            ac_image_urls: vec!["test_url".to_owned()],
            parts: Parts {
                weapons: Weapons {
                    r_arm: "Laser Blade".to_owned(),
                    l_arm: "Laser Blade".to_owned(),
                    r_back: "Laser Blade".to_owned(),
                    l_back: "Laser Blade".to_owned(),
                },
                frame: Frame {
                    head: "Head Type A".to_owned(),
                    core: "Core Type A".to_owned(),
                    arms: "Arms Type A".to_owned(),
                    legs: "Legs Type A".to_owned(),
                },
                inner: Inner {
                    booster: "Booster Type A".to_owned(),
                    fcs: "FCS Type A".to_owned(),
                    generator: "Generator Type A".to_owned(),
                },
                expansion: Some("Shield".to_string()),
            },
        };
        let user_id = 1;
        let id = repo.create(asm, user_id).await.unwrap();
        repo.delete(id).await.unwrap();
    }

    #[tokio::test]
    async fn test_read() {
        dotenv().ok();

        let db = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();

        let create_asm = AcAssembleNonId {
            user_id: 1,
            pilot_name: "test pilot".to_owned(),
            ac_name: "test ac ".to_owned(),
            description: "test description".to_owned(),
            remarks: "test remark".to_owned(),
            ac_card_image_url: "test_url".to_owned(),
            emblem_image_url: "test_url".to_owned(),
            ac_image_urls: vec!["test_url".to_owned()],
            parts: share::model::assemble_core::Parts {
                weapons: share::model::assemble_core::Weapons {
                    r_arm: "Laser Blade".to_owned(),
                    l_arm: "Laser Blade".to_owned(),
                    r_back: "Laser Blade".to_owned(),
                    l_back: "Laser Blade".to_owned(),
                },
                frame: share::model::assemble_core::Frame {
                    head: "Head Type A".to_owned(),
                    core: "Core Type A".to_owned(),
                    arms: "Arms Type A".to_owned(),
                    legs: "Legs Type A".to_owned(),
                },
                inner: Inner {
                    booster: "Booster Type A".to_owned(),
                    fcs: "FCS Type A".to_owned(),
                    generator: "Generator Type A".to_owned(),
                },
                expansion: Some("Shield".to_string()),
            },
        };
        let user_id = 1;

        let repo = Ac6AssembliesRepo::new(db);

        let id = repo.create(create_asm.clone(), user_id).await.unwrap();
        let read_asm = repo.read(id).await.unwrap();
        repo.delete(id).await.unwrap();

        assert_eq!(create_asm, read_asm.into());
    }

    #[tokio::test]
    async fn test_read_list() {
        dotenv().ok();

        let db = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();
        let create_asm = AcAssembleNonId {
            user_id: 1,
            pilot_name: "test pilot".to_owned(),
            ac_name: "test ac ".to_owned(),
            description: "test description".to_owned(),
            remarks: "test remark".to_owned(),
            ac_card_image_url: "test_url".to_owned(),
            emblem_image_url: "test_url".to_owned(),
            ac_image_urls: vec!["test_url".to_owned()],
            parts: share::model::assemble_core::Parts {
                weapons: share::model::assemble_core::Weapons {
                    r_arm: "Laser Blade".to_owned(),
                    l_arm: "Laser Blade".to_owned(),
                    r_back: "Laser Blade".to_owned(),
                    l_back: "Laser Blade".to_owned(),
                },
                frame: share::model::assemble_core::Frame {
                    head: "Head Type A".to_owned(),
                    core: "Core Type A".to_owned(),
                    arms: "Arms Type A".to_owned(),
                    legs: "Legs Type A".to_owned(),
                },
                inner: Inner {
                    booster: "Booster Type A".to_owned(),
                    fcs: "FCS Type A".to_owned(),
                    generator: "Generator Type A".to_owned(),
                },
                expansion: Some("Shield".to_string()),
            },
        };
        let user_id = 1;

        let repo = Ac6AssembliesRepo::new(db);
        let first_id = repo.create(create_asm.clone(), user_id).await.unwrap();
        for _ in 1..10 {
            repo.create(create_asm.clone(), user_id).await.unwrap();
        }

        let read_asm = repo.read_list(first_id, 10).await.unwrap();
        let mut read_asm_ids = read_asm.iter().map(|a| a.id).collect::<Vec<i32>>();
        let mut check_asm_ids = (first_id..first_id + 10).collect::<Vec<i32>>();
        let check_asm = (0..10)
            .map(|_| create_asm.clone())
            .collect::<Vec<AcAssembleNonId>>();
        let read_asm = read_asm
            .into_iter()
            .map(|a| a.into())
            .collect::<Vec<AcAssembleNonId>>();

        read_asm.iter().for_each(|a| {
            println!("{:?}", a);
        });

        assert_eq!(read_asm_ids.sort(), check_asm_ids.sort());
        assert_eq!(read_asm, check_asm);
    }

    #[tokio::test]
    async fn test_update() {
        dotenv().ok();

        let db = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();
        let repo = Ac6AssembliesRepo::new(db);

        let asm = AcAssembleNonId {
            user_id: 1,
            pilot_name: "test pilot".to_owned(),
            ac_name: "test ac ".to_owned(),
            description: "test description".to_owned(),
            remarks: "test remark".to_owned(),
            ac_card_image_url: "test_url".to_owned(),
            emblem_image_url: "test_url".to_owned(),
            ac_image_urls: vec!["test_url".to_owned()],
            parts: share::model::assemble_core::Parts {
                weapons: share::model::assemble_core::Weapons {
                    r_arm: "Laser Blade".to_owned(),
                    l_arm: "Laser Blade".to_owned(),
                    r_back: "Laser Blade".to_owned(),
                    l_back: "Laser Blade".to_owned(),
                },
                frame: share::model::assemble_core::Frame {
                    head: "Head Type A".to_owned(),
                    core: "Core Type A".to_owned(),
                    arms: "Arms Type A".to_owned(),
                    legs: "Legs Type A".to_owned(),
                },
                inner: Inner {
                    booster: "Booster Type A".to_owned(),
                    fcs: "FCS Type A".to_owned(),
                    generator: "Generator Type A".to_owned(),
                },
                expansion: Some("Shield".to_string()),
            },
        };
        let user_id = 1;
        let id = repo.create(asm, user_id).await.unwrap();

        let asm = AcAssemble {
            id,
            user_id: 1,
            pilot_name: "test2 pilot".to_owned(),
            ac_name: "test2 ac ".to_owned(),
            description: "test2 description".to_owned(),
            remarks: "test2 remark".to_owned(),
            ac_card_image_url: "test_url2".to_owned(),
            emblem_image_url: "test_url2".to_owned(),
            ac_image_urls: vec!["test_url2".to_owned()],
            parts: share::model::assemble_core::Parts {
                weapons: share::model::assemble_core::Weapons {
                    r_arm: "Laser Blade".to_owned(),
                    l_arm: "Laser Blade".to_owned(),
                    r_back: "Laser Blade".to_owned(),
                    l_back: "Laser Blade".to_owned(),
                },
                frame: share::model::assemble_core::Frame {
                    head: "Head Type B".to_owned(),
                    core: "Core Type B".to_owned(),
                    arms: "Arms Type B".to_owned(),
                    legs: "Legs Type B".to_owned(),
                },
                inner: Inner {
                    booster: "Booster Type A".to_owned(),
                    fcs: "FCS Type A".to_owned(),
                    generator: "Generator Type A".to_owned(),
                },
                expansion: Some("Shield".to_string()),
            },
        };
        repo.update(asm.clone()).await.unwrap();
        let read_asm = repo.read(id).await.unwrap();
        repo.delete(id).await.unwrap();

        assert_eq!(asm, read_asm);
    }

    #[tokio::test]
    async fn test_delete() {
        dotenv().ok();

        let db = PgPoolOptions::new()
            .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();
        let repo = Ac6AssembliesRepo::new(db);

        let asm = AcAssembleNonId {
            user_id: 1,
            pilot_name: "test pilot".to_owned(),
            ac_name: "test ac ".to_owned(),
            description: "test description".to_owned(),
            remarks: "test remark".to_owned(),
            ac_card_image_url: "test_url".to_owned(),
            emblem_image_url: "test_url".to_owned(),
            ac_image_urls: vec!["test_url".to_owned()],
            parts: share::model::assemble_core::Parts {
                weapons: share::model::assemble_core::Weapons {
                    r_arm: "Laser Blade".to_owned(),
                    l_arm: "Laser Blade".to_owned(),
                    r_back: "Laser Blade".to_owned(),
                    l_back: "Laser Blade".to_owned(),
                },
                frame: share::model::assemble_core::Frame {
                    head: "Head Type A".to_owned(),
                    core: "Core Type A".to_owned(),
                    arms: "Arms Type A".to_owned(),
                    legs: "Legs Type A".to_owned(),
                },
                inner: Inner {
                    booster: "Booster Type A".to_owned(),
                    fcs: "FCS Type A".to_owned(),
                    generator: "Generator Type A".to_owned(),
                },
                expansion: Some("Shield".to_string()),
            },
        };
        let user_id = 1;
        let id = repo.create(asm, user_id).await.unwrap();
        repo.delete(id).await.unwrap();
    }

    // テストデータを削除する
    // 関数の都合，テストデータを削除する際には，テストデータのidを指定する必要がある
    // createテストはdeleteの正常動作に依存し，deleteテストはcreateの正常動作に依存するため，両方壊れるとうまくテストできない
    // createとdeleteが同時に不具合が起きた場合，この手動でidを指定するテストを使ってテストする
    // #[tokio::test]
    // async fn test_delete_manual() {
    //     dotenv().ok();

    //     let db = PgPoolOptions::new()
    //     .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
    //     .await
    //     .unwrap();
    //     let repo = Ac6AssembliesRepo::new(db);

    //     let id = 6;
    //     repo.delete(id).await.unwrap();
    // }
}
