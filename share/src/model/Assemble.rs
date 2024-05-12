use typeshare::typeshare;

#[typeshare]
pub struct Weapons {
    r_arm: String,
    l_arm: String,
    r_back: String,
    l_back: String,
}


#[typeshare]
pub enum TestEnum {
    HandGan,
    ShotGan,
}