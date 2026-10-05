//! Spyro moveset plugin - v0.1
//! Step 1: register Spyro on the character select screen (CSK Collection),
//! as a duplicate of Duck Hunt using Duck Hunt's costume slots c120-c127.
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
    // Which costume folder each of Spyro's 8 colors loads: color 0 -> c120 ... color 7 -> c127
    let mut index_maps: HashMap<u64, UnsignedByteType> = HashMap::new();
    for i in 0..COLOR_NUM {
        index_maps.insert(hash40(&format!("c{:02}_index", i)), UnsignedByteType::Overwrite(COLOR_START + i));
    }

    add_chara_db_entry_info(CharacterDatabaseEntry {
        ui_chara_id: hash40("ui_chara_spyro"),
        clone_from_ui_chara_id: Some(hash40("ui_chara_duckhunt")),
        // "spyro" is what the name labels (nam_chr1_00_spyro ...) and portraits (chara_1_spyro_00 ...) use
        name_id: StringType::Overwrite(CStrCSK::new("spyro")),
        color_num: UnsignedByteType::Overwrite(COLOR_NUM),
        extra_index_maps: UnsignedByteMap::Overwrite(index_maps),
        ..Default::default()
    });

    // Portrait placement on the select screen: start from Duck Hunt's layout for each color
    for i in 0..COLOR_NUM {
        add_chara_layout_db_entry_info(CharacterLayoutDatabaseEntry {
            ui_layout_id: hash40(&format!("ui_chara_spyro_{:02}", i)),
            clone_from_ui_layout_id: Some(hash40(&format!("ui_chara_duckhunt_{:02}", i))),
            ui_chara_id: Hash40Type::Overwrite(hash40("ui_chara_spyro")),
            ..Default::default()
        });
    }
}

#[skyline::main(name = "spyro")]
pub fn main() {
    register_select_screen();
}
