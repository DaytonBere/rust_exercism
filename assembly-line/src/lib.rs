// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
const PROD_RATE: f64 = 221.0;
const ONE_FOUR: f64 = 1.00;
const FIVE_EIGHT: f64 = 0.90;
const NINE_TEN: f64 = 0.77;


pub fn production_rate_per_hour(speed: u8) -> f64 {
    let PASS: f64;
    match speed {
        1|2|3|4=> (PASS = ONE_FOUR),
        5|6|7|8=> (PASS = FIVE_EIGHT),
        9|10=> (PASS = NINE_TEN),
        _=> (PASS = 0.0),
    }
    (speed as f64) * PASS * PROD_RATE
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
