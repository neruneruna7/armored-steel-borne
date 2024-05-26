use anyhow::Result;
use serde::Deserialize;
use share::model::assemble_core::{AcAssemble, AcAssembleNonId};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Json;
use sqlx::FromRow;
use sqlx::PgPool;

#[derive(Debug, Deserialize, FromRow)]
pub struct Ac6AssemblyRead {
    pub id: i32,
    pub pilot_name: String,
    pub ac_name: String,
    pub description: String,
    pub remarks: String,
    pub ac_card_image_url: String,
    pub emblem_image_url: String,
    pub ac_image_urls: Json<Vec<String>>,
    pub l_arm_name: String,
    pub r_arm_name: String,
    pub l_back_name: String,
    pub r_back_name: String,
    pub head_name: String,
    pub core_name: String,
    pub arms_name: String,
    pub legs_name: String,
    pub boost_name: String,
    pub fcs_name: String,
    pub generator_name: String,
    pub expansion_name: Option<String>,
    pub user_id: i32,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Ac6AssemblyInsert {
    pub pilot_name: String,
    pub ac_name: String,
    pub description: String,
    pub remarks: String,
    pub ac_card_image_url: String,
    pub emblem_image_url: String,
    pub ac_image_urls: Vec<String>,
    pub l_arm_name: String,
    pub r_arm_name: String,
    pub l_back_name: String,
    pub r_back_name: String,
    pub head_name: String,
    pub core_name: String,
    pub arms_name: String,
    pub legs_name: String,
    pub boost_name: String,
    pub fcs_name: String,
    pub generator_name: String,
    pub expansion_name: Option<String>,
    pub user_id: i32,
}

impl Ac6AssemblyInsert {
    pub fn new(
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
        boost_name: String,
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
            boost_name,
            fcs_name,
            generator_name,
            expansion_name,
            user_id,
        }
    }

    // インナーパーツはまだ未実装
    pub fn from_acasm_nonid(value: AcAssembleNonId, user_id: i32) -> Self {
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
            "Booster Type A".to_owned(),
            "FCS Type A".to_owned(),
            "Generator Type A".to_owned(),
            Some("Shield".to_owned()),
            user_id,
        )
    }
}

pub struct Ac6AssembliesRepo {
    db: PgPool,
}

/// 新しくレコードを挿入する際，RETURNINGでidを取得するための構造体
/// query_as!マクロが構造体しかとらない仕様のため，こうやって構造体を定義している
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
                boost_name,
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
            asm.boost_name,
            asm.fcs_name,
            asm.generator_name,
            asm.expansion_name,
            asm.user_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(r.id)
    }

    pub async fn read(&self, id: i32) -> Result<Ac6AssemblyRead> {
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
                boost_name,
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
        Ok(asm)
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
    use share::model::assemble_core::AcAssembleNonId;
    use sqlx::postgres::PgPoolOptions;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_create() {
        dotenv().ok();

        let db = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .unwrap();
        let repo = Ac6AssembliesRepo::new(db);

        let asm = AcAssembleNonId {
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
            },
        };
        let user_id = 1;
        let id = repo.create(asm, user_id).await.unwrap();
        panic!("id: {}", id);
    }

    #[tokio::test]
    async fn test_read() {
        dotenv().ok();

        let db = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .unwrap();
        let repo = Ac6AssembliesRepo::new(db);

        let id = 1;
        let asm = repo.read(id).await.unwrap();
        panic!("asm: {:?}", asm);
    }

    #[tokio::test]
    async fn test_delete() {
        dotenv().ok();

        let db = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .unwrap();
        let repo = Ac6AssembliesRepo::new(db);

        let id = 6;
        repo.delete(id).await.unwrap();
    }
}