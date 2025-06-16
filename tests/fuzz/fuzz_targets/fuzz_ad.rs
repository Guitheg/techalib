#![no_main]

use libfuzzer_sys::fuzz_target;
use techalib::{indicators::ad::ad, types::Float};

fuzz_target!(|data: (Vec<Float>, Vec<Float>, Vec<Float>, Vec<Float>)| {
    let (high, low, close, volume) = data;
    let _ = ad(&high, &low, &close, &volume);
});
