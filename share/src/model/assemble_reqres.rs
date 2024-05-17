use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::NoneAsEmptyString;
use typeshare::typeshare;
use ulid::Ulid;
use utoipa::IntoParams;
use utoipa::ToSchema;

use super::assemble_core::AcAssemble;
use super::assemble_core::AcAssembleNonUlid;

#[typeshare]
// #[derive(Deserialize, ToSchema)]
// #[serde(rename_all = "camelCase")]
pub type AcAsmGetReq = Ulid;

#[typeshare]
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAsmGetRes {
    pub ac_assemble: AcAssemble,
}

#[serde_as]
#[typeshare]
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase", default)]
pub struct AcAsmListReq {
    #[serde_as(as = "NoneAsEmptyString")]
    pub prev_id: Option<Ulid>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub size: Option<u32>,
}

impl Default for AcAsmListReq {
    fn default() -> Self {
        Self {
            prev_id: None,
            size: Some(20),
        }
    }
}

#[typeshare]
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAsmListRes {
    pub ac_assembles: Vec<AcAssemble>,
}


#[typeshare]
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AcAsmUpdateReq {
    pub ac_assemble: AcAssemble,
}

// #[typeshare]
// #[derive(Debug, Clone, Serialize, IntoParams)]
// #[serde(rename_all = "camelCase")]
// pub struct AcAsmUpdateRes;

#[typeshare]
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAsmPostReq {
    pub ac_assemble: AcAssembleNonUlid,
}
