use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use utoipa::ToSchema;

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Weapons {
    pub r_arm: String,
    pub l_arm: String,
    pub r_back: String,
    pub l_back: String,
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub head: String,
    pub core: String,
    pub arms: String,
    pub legs: String,
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Inner {
    pub booster: String,
    pub fcs: String,
    pub generator: String,
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Parts {
    pub weapons: Weapons,
    pub frame: Frame,
    pub inner: Inner,
    pub expansion: Option<String>,
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAssemble {
    pub id: i32,
    pub user_id: i32,
    pub pilot_name: String,
    pub ac_name: String,
    pub ac_card_image_url: String,
    pub emblem_image_url: String,
    pub ac_image_urls: Vec<String>,
    pub parts: Parts,
    pub description: String,
    pub remarks: String,
}

impl From<AcAssemble> for AcAssembleNonId {
    fn from(val: AcAssemble) -> Self {
        AcAssembleNonId {
            user_id: val.user_id,
            pilot_name: val.pilot_name,
            ac_name: val.ac_name,
            ac_card_image_url: val.ac_card_image_url,
            emblem_image_url: val.emblem_image_url,
            ac_image_urls: val.ac_image_urls,
            parts: val.parts,
            description: val.description,
            remarks: val.remarks,
        }
    }
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAssembleNonId {
    pub user_id: i32,
    pub pilot_name: String,
    pub ac_name: String,
    pub ac_card_image_url: String,
    pub emblem_image_url: String,
    pub ac_image_urls: Vec<String>,
    pub parts: Parts,
    pub description: String,
    pub remarks: String,
}
