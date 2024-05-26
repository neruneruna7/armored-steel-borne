use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use ulid::Ulid;
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
#[derive(Debug, Clone, PartialEq, Eq,  Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Parts {
    pub weapons: Weapons,
    pub frame: Frame,
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAssemble {
    pub id: i32,
    pub pilot_name: String,
    pub ac_name: String,
    pub ac_card_image_url: String,
    pub emblem_image_url: String,
    pub ac_image_urls: Vec<String>,
    pub parts: Parts,
    pub description: String,
    pub remarks: String,
}

impl Into<AcAssembleNonId> for AcAssemble {
    fn into(self) -> AcAssembleNonId {
        AcAssembleNonId {
            pilot_name: self.pilot_name,
            ac_name: self.ac_name,
            ac_card_image_url: self.ac_card_image_url,
            emblem_image_url: self.emblem_image_url,
            ac_image_urls: self.ac_image_urls,
            parts: self.parts,
            description: self.description,
            remarks: self.remarks,
        }
    }
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq ,Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcAssembleNonId {
    pub pilot_name: String,
    pub ac_name: String,
    pub ac_card_image_url: String,
    pub emblem_image_url: String,
    pub ac_image_urls: Vec<String>,
    pub parts: Parts,
    pub description: String,
    pub remarks: String,
}
