use typeshare::typeshare;
use ulid::Ulid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[typeshare]
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Weapons {
    pub r_arm: String,
    pub l_arm: String,
    pub r_back: String,
    pub l_back: String,
}


#[typeshare]
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub head: String,
    pub core: String,
    pub arms: String,
    pub legs: String,
}

#[typeshare]
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Parts {
    pub weapons: Weapons,
    pub frame: Frame,
}

#[typeshare]
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAssemble {
pub     ulid: Ulid,
pub     pilot_name: String,
pub     ac_name: String,
pub     ac_card_image_url: String,
pub     emblem_image_url: String,
pub     ac_image_urls: Vec<String>,
pub     parts: Parts,
pub     description: String,
pub     remarks: String,
}

#[typeshare]
// #[derive(Deserialize, ToSchema)]
// #[serde(rename_all = "camelCase")]
pub type AcAsmGetReq = Ulid;

#[typeshare]
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAsmGetRes {
    pub ac_assemble: AcAssemble
}


#[typeshare]
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAsmListReq {
    pub prev_id: Option<Ulid>,
    pub size: Option<u32>
}

#[typeshare]
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAsmListRes {
    pub ac_assembles: Vec<AcAssemble>
}
