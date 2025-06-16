use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data_eps, load_generated_csv},
};

use crate::expect_err_overflow_or_ok_with;
use techalib::{
    errors::TechalibError,
    indicators::ad::{ad, ad_into, AdResult, AdSample},
    traits::State,
    types::Float,
};

fn generated_and_no_lookahead_ad(file_name: &str) {
    let columns = load_generated_csv(file_name).unwrap();
    let high = columns.get("high").unwrap();
    let low = columns.get("low").unwrap();
    let close = columns.get("close").unwrap();
    let volume = columns.get("volume").unwrap();

    let len = high.len();
    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected = columns.get("ad").unwrap();

    let input_high = &high[0..last_idx];
    let input_low = &low[0..last_idx];
    let input_close = &close[0..last_idx];
    let input_volume = &volume[0..last_idx];

    let output = ad(input_high, input_low, input_close, input_volume);
    assert!(output.is_ok(), "Failed to calculate AD: {:?}", output.err());
    let result = output.unwrap();

    assert_vec_eq_gen_data_eps(&expected[0..last_idx], &result.ad, 1e-6);

    let mut new_state = result.state;
    for i in 0..next_count {
        let sample = AdSample {
            high: high[last_idx + i],
            low: low[last_idx + i],
            close: close[last_idx + i],
            volume: volume[last_idx + i],
        };
        new_state.update(&sample).unwrap();
        assert!(
            approx_eq_float(new_state.ad, expected[last_idx + i], 1e-8),
            "Next [{}] expected {}, but got {}",
            i,
            expected[last_idx + i],
            new_state.ad
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_ad("ad.csv")
}

#[test]
fn finite_extreme_err_overflow_or_ok_all_finite() {
    let high = vec![
        Float::MAX - 300.0,
        Float::MAX - 220.0,
        Float::MAX - 500.0,
        Float::MAX - 600.0,
        Float::MAX - 800.0,
        Float::MAX - 100.0,
    ];
    let close = vec![
        Float::MAX - 400.0,
        Float::MAX - 280.0,
        Float::MAX - 560.0,
        Float::MAX - 700.0,
        Float::MAX - 850.0,
        Float::MAX - 150.0,
    ];
    let low = vec![
        Float::MAX - 500.0,
        Float::MAX - 320.0,
        Float::MAX - 700.0,
        Float::MAX - 800.0,
        Float::MAX - 900.0,
        Float::MAX - 300.0,
    ];
    let volume = vec![
        Float::MAX - 3.0,
        Float::MAX - 2.0,
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
    ];
    let period = 3;
    expect_err_overflow_or_ok_with!(ad(&high, &low, &close, &volume), |result: AdResult| {
        assert!(
            result.ad.iter().skip(period).all(|v| v.is_finite()),
            "Expected all values to be finite"
        );
    });
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let high = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let low = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let close = vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let volume = vec![100.0, 200.0, 300.0, 400.0, 350.0, 250.0, 150.0];
    let result = ad(&high, &low, &close, &volume).unwrap();
    let mut state = result.state;
    let sample = AdSample {
        high: Float::MIN + 5.0,
        low: Float::MIN + 2.0,
        close: Float::MIN + 3.0,
        volume: 100.0,
    };
    expect_err_overflow_or_ok_with!(state.update(&sample), |_| {
        assert!(state.ad.is_finite(), "Expected all values to be finite");
    });
}

#[test]
fn unexpected_nan_err() {
    let high = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let low = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let close = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let volume = vec![100.0, 200.0, 300.0, 400.0, 350.0, 250.0, 150.0];
    let result = ad(&high, &low, &close, &volume);
    assert!(result.is_err());
    assert!(
        matches!(result, Err(TechalibError::DataNonFinite(_))),
        "Expected an error for non-finite data, got: {:?}",
        result
    );
}

#[test]
fn non_finite_err() {
    let high = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let low = vec![1.0, 2.0, 3.0, 4.0, Float::NEG_INFINITY, 6.0];
    let close = vec![1.0, 2.0, 3.0, 4.0, 5.0, Float::NAN];
    let volume = vec![100.0, 200.0, 300.0, 400.0, 350.0, 250.0];
    let result = ad(&high, &low, &close, &volume);
    assert!(
        result.is_err(),
        "Expected an error for non-finite data, got: {:?}",
        result
    );
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn empty_input_err() {
    let high: [Float; 0] = [];
    let low: [Float; 0] = [];
    let close: [Float; 0] = [];
    let volume: [Float; 0] = [];
    let result = ad(&high, &low, &close, &volume);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::InsufficientData)));
}

#[test]
fn different_length_input_output_err() {
    let high = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let low = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let close = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let volume = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut output = vec![0.0; 3];
    let result = ad_into(&high, &low, &close, &volume, output.as_mut_slice());
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}

#[test]
fn different_length_inputs_err() {
    let high = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let low = vec![1.0, 2.0, 3.0];
    let close = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let volume = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = ad(&high, &low, &close, &volume);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}

#[test]
fn same_high_low_returns_all_0_ok() {
    let high = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let low = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let close = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let volume = vec![100.0, 200.0, 300.0, 400.0, 500.0];
    let result = ad(&high, &low, &close, &volume).unwrap();
    assert!(
        result.ad.iter().all(|&v| v == 0.0),
        "Expected all values to be 0.0"
    );
}
