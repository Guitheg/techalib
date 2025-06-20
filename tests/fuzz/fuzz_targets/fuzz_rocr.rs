#![no_main]

use libfuzzer_sys::fuzz_target;
use techalib::{indicators::rocr::rocr, types::Float};

fuzz_target!(|data: (Vec<Float>, usize)| {
    let (v, period) = data;
    let len = v.len();
    let period = (period as usize % len.saturating_add(1)).max(1);
    let _ = rocr(&v, period);
});
