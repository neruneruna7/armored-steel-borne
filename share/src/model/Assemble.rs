use typeshare::typeshare;
use ulid::Ulid;
use serde::Serialize;

#[typeshare]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Weapons {
    r_arm: String,
    l_arm: String,
    r_back: String,
    l_back: String,
}


#[typeshare]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    head: String,
    core: String,
    arms: String,
    legs: String,
}

#[typeshare]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parts {
    weapons: Weapons,
    frame: Frame,
}

#[typeshare]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcAssemble {
    ulid: Ulid,
    pilot_name: String,
    ac_name: String,
    ac_card_image_url: String,
    emblem_image_url: String,
    ac_image_urls: Vec<String>,
    parts: Parts,
    description: String,
    remarks: String,
}



