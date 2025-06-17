#![no_main]

use libfuzzer_sys::fuzz_target;
use techalib::{indicators::minus_dm::minus_dm, types::Float};

fuzz_target!(|data: (Vec<Float>, Vec<Float>, u8)| {
    let (high, low, period) = data;
    let period = (period as usize % high.len().saturating_add(1)).max(1);
    let _ = minus_dm(&high, &low, period);
});
