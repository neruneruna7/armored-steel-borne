-- Add migration script here
-- 武器マスターテーブル
CREATE TABLE  IF NOT EXISTS ac6_weapons_master (
    weapon_name VARCHAR(50) PRIMARY KEY,
    info VARCHAR(200) NOT NULL
);

-- フレームパーツマスターテーブル（頭部）
CREATE TABLE  IF NOT EXISTS ac6_frame_heads_master (
    head_name VARCHAR(50) PRIMARY KEY,
    info VARCHAR(200) NOT NULL
);

-- フレームパーツマスターテーブル（コア部分）
CREATE TABLE  IF NOT EXISTS ac6_frame_cores_master (
    core_name VARCHAR(50) PRIMARY KEY,
    info VARCHAR(200) NOT NULL
);

-- フレームパーツマスターテーブル（腕部）
CREATE TABLE  IF NOT EXISTS ac6_frame_arms_master (
    arms_name VARCHAR(50) PRIMARY KEY,
    info VARCHAR(200) NOT NULL
);

-- フレームパーツマスターテーブル（脚部）
CREATE TABLE  IF NOT EXISTS ac6_frame_legs_master (
    legs_name VARCHAR(50) PRIMARY KEY,
    info VARCHAR(200) NOT NULL
);

-- インナーパーツマスターテーブル（ブースター）
CREATE TABLE  IF NOT EXISTS ac6_inner_boosters_master (
    booster_name VARCHAR(50) PRIMARY KEY,
    info VARCHAR(200) NOT NULL
);

-- インナーパーツマスターテーブル（FCS）
CREATE TABLE  IF NOT EXISTS ac6_inner_fcs_master (
    fcs_name VARCHAR(50) PRIMARY KEY,
    info VARCHAR(200) NOT NULL
);

-- インナーパーツマスターテーブル（ジェネレーター）
CREATE TABLE  IF NOT EXISTS ac6_inner_generators_master (
    generator_name VARCHAR(50) PRIMARY KEY,
    info VARCHAR(200) NOT NULL
);

-- エキスパンションパーツマスターテーブル
CREATE TABLE  IF NOT EXISTS ac6_expansion_parts_master (
    expansion_name VARCHAR(50) PRIMARY KEY,
    info VARCHAR(200) NOT NULL
);


-- アセンブリテーブル
CREATE TABLE  IF NOT EXISTS ac6_assemblies (
    id SERIAL PRIMARY KEY,
    pilot_name TEXT NOT NULL,
    ac_name TEXT NOT NULL,
    description TEXT NOT NULL,
    remarks TEXT NOT NULL,
    ac_card_image_url TEXT NOT NULL,
    emblem_image_url TEXT NOT NULL,
    ac_image_urls TEXT[] NOT NULL,
    l_arm_name VARCHAR(50) NOT NULL,
    r_arm_name VARCHAR(50) NOT NULL,
    l_back_name VARCHAR(50) NOT NULL,
    r_back_name VARCHAR(50) NOT NULL,
    head_name VARCHAR(50) NOT NULL,
    core_name VARCHAR(50) NOT NULL,
    arms_name VARCHAR(50) NOT NULL,
    legs_name VARCHAR(50) NOT NULL,
    boost_name VARCHAR(50) NOT NULL,
    fcs_name VARCHAR(50) NOT NULL,
    generator_name VARCHAR(50) NOT NULL,
    expansion_name VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    user_id INT NOT NULL,
    FOREIGN KEY (l_arm_name) REFERENCES ac6_weapons_master(weapon_name),
    FOREIGN KEY (r_arm_name) REFERENCES ac6_weapons_master(weapon_name),
    FOREIGN KEY (l_back_name) REFERENCES ac6_weapons_master(weapon_name),
    FOREIGN KEY (r_back_name) REFERENCES ac6_weapons_master(weapon_name),
    FOREIGN KEY (head_name) REFERENCES ac6_frame_heads_master(head_name),
    FOREIGN KEY (core_name) REFERENCES ac6_frame_cores_master(core_name),
    FOREIGN KEY (arms_name) REFERENCES ac6_frame_arms_master(arms_name),
    FOREIGN KEY (legs_name) REFERENCES ac6_frame_legs_master(legs_name),
    FOREIGN KEY (boost_name) REFERENCES ac6_inner_boosters_master(booster_name),
    FOREIGN KEY (fcs_name) REFERENCES ac6_inner_fcs_master(fcs_name),
    FOREIGN KEY (generator_name) REFERENCES ac6_inner_generators_master(generator_name),
    FOREIGN KEY (expansion_name) REFERENCES ac6_expansion_parts_master(expansion_name),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
