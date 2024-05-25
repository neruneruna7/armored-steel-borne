-- Add migration script here
-- V4__Insert_Dummy_Data.sql

-- マスターテーブルにダミーデータを挿入
INSERT INTO ac6_weapons_master (weapon_name, info) VALUES
('Laser Blade', 'A powerful laser blade for close combat.'),
('Machine Gun', 'A rapid-fire machine gun.'),
('Grenade Launcher', 'A high-explosive grenade launcher.'),
('Missile Launcher', 'A missile launcher with tracking capability.');

INSERT INTO ac6_frame_heads_master (head_name, info) VALUES
('Head Type A', 'Standard head with balanced attributes.'),
('Head Type B', 'Enhanced sensor head.');

INSERT INTO ac6_frame_cores_master (core_name, info) VALUES
('Core Type A', 'Standard core with balanced attributes.'),
('Core Type B', 'Heavy core with high defense.');

INSERT INTO ac6_frame_arms_master (arms_name, info) VALUES
('Arms Type A', 'Standard arms with balanced attributes.'),
('Arms Type B', 'Heavy arms with high weapon capacity.');

INSERT INTO ac6_frame_legs_master (legs_name, info) VALUES
('Legs Type A', 'Standard legs with balanced attributes.'),
('Legs Type B', 'Heavy legs with high load capacity.');

INSERT INTO ac6_inner_boosters_master (booster_name, info) VALUES
('Booster Type A', 'Standard booster with balanced attributes.'),
('Booster Type B', 'High-speed booster.');

INSERT INTO ac6_inner_fcs_master (fcs_name, info) VALUES
('FCS Type A', 'Standard fire control system.'),
('FCS Type B', 'Advanced fire control system.');

INSERT INTO ac6_inner_generators_master (generator_name, info) VALUES
('Generator Type A', 'Standard generator with balanced output.'),
('Generator Type B', 'High-output generator.');

INSERT INTO ac6_expansion_parts_master (expansion_name, info) VALUES
('Shield', 'Defensive shield for extra protection.'),
('Cannon', 'High-powered cannon for additional firepower.');

-- ユーザーテーブルにダミーデータを挿入
INSERT INTO users (name, email, password, created_at, role) VALUES
('User1', 'user1@example.com', 'password1', CURRENT_TIMESTAMP, 'user'),
('User2', 'user2@example.com', 'password2', CURRENT_TIMESTAMP, 'user'),
('Admin1', 'admin1@example.com', 'adminpassword', CURRENT_TIMESTAMP, 'admin'),
('User3', 'user3@example.com', 'password3', CURRENT_TIMESTAMP, 'user'),
('User4', 'user4@example.com', 'password4', CURRENT_TIMESTAMP, 'user');


-- アセンブリテーブルにダミーデータを挿入
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
    created_at,
    updated_at,
    user_id
) VALUES
(
    'Pilot A', 
    'AC Name A', 
    'Description A', 
    'Remarks A', 
    '/ac/steel-haze.webp', 
    '/ac/rusty.jpg', 
    ARRAY['/ac/steel-haze.webp', '/ac/ac.jpg', '/ac/ac2.jpg', '/ac/ac3.png'],
    'Laser Blade',    -- 左腕武器
    'Machine Gun',    -- 右腕武器
    'Grenade Launcher', -- 左肩武器
    'Missile Launcher', -- 右肩武器
    'Head Type A',    -- 頭部パーツ
    'Core Type A',    -- コアパーツ
    'Arms Type A',    -- 腕パーツ
    'Legs Type A',    -- 脚パーツ
    'Booster Type A', -- ブースター
    'FCS Type A',     -- FCS
    'Generator Type A', -- ジェネレーター
    'Shield',         -- エキスパンションパーツ
    CURRENT_TIMESTAMP, -- 作成日時
    CURRENT_TIMESTAMP, -- 更新日時
    1                 -- ユーザーID
),
(
    'Pilot B', 
    'AC Name B', 
    'Description B', 
    'Remarks B', 
    '/ac/liger-tail.jpg', 
    '/ac/rusty.jpg', 
    ARRAY['/ac/liger-tail.jpg', '/ac/liger-tail-2.jpg', '/ac/liger-tail-3.jpg'],
    'Machine Gun',    -- 左腕武器
    'Laser Blade',    -- 右腕武器
    'Missile Launcher', -- 左肩武器
    'Grenade Launcher', -- 右肩武器
    'Head Type B',    -- 頭部パーツ
    'Core Type B',    -- コアパーツ
    'Arms Type B',    -- 腕パーツ
    'Legs Type B',    -- 脚パーツ
    'Booster Type B', -- ブースター
    'FCS Type B',     -- FCS
    'Generator Type B', -- ジェネレーター
    'Cannon',         -- エキスパンションパーツ
    CURRENT_TIMESTAMP, -- 作成日時
    CURRENT_TIMESTAMP, -- 更新日時
    2                 -- ユーザーID
);