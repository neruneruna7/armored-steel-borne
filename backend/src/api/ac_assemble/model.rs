use serde::{Deserialize, Serialize};
use share::model::Assemble::AcAssemble;
use ulid::Ulid;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct AcAsmGetReq {
    pub ulid: Ulid
}

#[derive(Serialize, ToSchema)]
pub(crate) struct AcAsmGetRes {
    pub ac_assemble: AcAssemble
}
