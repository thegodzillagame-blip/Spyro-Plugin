//! Spyro moveset plugin - v0.4

#![allow(unused)]

use std::collections::HashMap;
use the_csk_collection_api::*;

/// First costume slot Spyro uses on Duck Hunt (c120), and how many colors he has.
const COLOR_START: u8 = 120;
const COLOR_NUM: u8 = 8;

/// Smash's hash40: CRC32 of the lowercase string, with the length in the top bits.
fn crc32(data: &[u8]) -> u32 {
    let mut c: u32 = 0xFFFF_FFFF;
    for &b in data {
        c ^= b as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ 0xEDB8_8320 } else { c >> 1 };
        }
    }
    !c
}
pub fn hash40(s: &str) -> u64 {
    let lower = s.to_lowercase();
    ((lower.len() as u64) << 32) | crc32(lower.as_bytes()) as u64
}

fn register_select_screen() {
    // The CSK Collection reads c00_index as the first costume folder of this select-screen icon;
    // the other colors follow it in order (c120 ... c127). Setting c01-c07 as well pointed the
    // select screen at portrait layouts that don't exist, which froze it while loading.
    let mut index_maps: HashMap<u64, UnsignedByteType> = HashMap::new();
    index_maps.insert(hash40("c00_index"), UnsignedByteType::Overwrite(0)); // TEST v0.4: portrait index 0 (was 120)

    add_chara_db_entry_info(CharacterDatabaseEntry {
        ui_chara_id: hash40("ui_chara_spyro"),
        clone_from_ui_chara_id: Some(hash40("ui_chara_duckhunt")),
        // "spyro" is what the name labels (nam_chr1_00_spyro ...) and portraits (chara_1_spyro_00 ...) use
        name_id: StringType::Overwrite(CStrCSK::new("spyro")),
        color_num: UnsignedByteType::Overwrite(COLOR_NUM),
        extra_index_maps: UnsignedByteMap::Overwrite(index_maps),
        ..Default::default()
    });

    // Duck Hunt has exactly one portrait layout (ui_chara_duckhunt_00) shared by all his colors.
    add_chara_layout_db_entry_info(CharacterLayoutDatabaseEntry {
        ui_layout_id: hash40("ui_chara_spyro_00"),
        clone_from_ui_layout_id: Some(hash40("ui_chara_duckhunt_00")),
        ui_chara_id: Hash40Type::Overwrite(hash40("ui_chara_spyro")),
        ..Default::default()
    });
}

/// Spyro's physical stats on his costumes (c120-c127). Floats multiply Duck Hunt's values.
fn install_stats() {
    unsafe {
        let kind: i32 = *smash::lib::lua_const::FIGHTER_KIND_DUCKHUNT;
        let slots: Vec<i32> = (COLOR_START as i32..(COLOR_START + COLOR_NUM) as i32).collect();
        // 3 jumps total (ground jump + 2 in the air)
        param_config::update_int_2(kind, slots.clone(), (hash40("jump_count_max"), 0, 3));
        for (name, mul) in [
            ("weight", 1.1395f32),            // 86 -> 98
            ("walk_speed_max", 1.0717),       // 1.213 -> 1.30
            ("run_speed_max", 1.1154),        // 1.793 -> 2.00
            ("air_speed_x_stable", 1.0476),   // 1.155 -> 1.21
            ("air_speed_y_stable", 0.9394),   // fall speed 1.65 -> 1.55
            ("dive_speed_y", 1.0606),         // fast fall 2.64 -> 2.80
        ] {
            param_config::update_attribute_mul_2(kind, slots.clone(), (hash40(name), 0, mul));
        }
    }
}

#[skyline::main(name = "spyro")]
pub fn main() {
    register_select_screen();
    install_stats();
}

