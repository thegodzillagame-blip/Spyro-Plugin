//! Spyro (replaces Duck Hunt) - plugin v0.7

#![allow(unused)]

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

/// All 8 of Duck Hunt's costumes are Spyro.
fn install_stats() {
    unsafe {
        let kind: i32 = *smash::lib::lua_const::FIGHTER_KIND_DUCKHUNT;
        let slots: Vec<i32> = (0..8).collect();
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
    install_stats();
}
