#![no_main]

use libfuzzer_sys::fuzz_target;
use techalib::{indicators::atr::atr, types::Float};

fuzz_target!(|data: (Vec<Float>, Vec<Float>, Vec<Float>, u8)| {
    let (close, high, low, period) = data;
    let period = (period as usize % close.len().saturating_add(1)).max(1);
    let _ = atr(&close, &high, &low, period);
});
