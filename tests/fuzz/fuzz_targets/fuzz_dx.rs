#![no_main]

use libfuzzer_sys::fuzz_target;
use techalib::{indicators::dx::dx, types::Float};

fuzz_target!(|data: (Vec<Float>, Vec<Float>, Vec<Float>, usize)| {
    let (high, low, close, period) = data;
    let len = high.len();
    let period = (period as usize % len.saturating_add(1)).max(1);
    let _ = dx(&high, &low, &close, period);
});
