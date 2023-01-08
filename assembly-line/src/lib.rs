// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate_per_hour:f64 = (speed as i32 * 221) as f64;
    match speed {
        1..=4 => {
           rate_per_hour 
        },
        5..=8 => {
            rate_per_hour * 0.9
        }
        9..=10 => {
            rate_per_hour * 0.77
        }
        _ => {
            rate_per_hour
        }
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate_per_hour = production_rate_per_hour(speed);
    (rate_per_hour / 60.0) as u32
}
