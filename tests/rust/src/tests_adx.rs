use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use crate::expect_err_overflow_or_ok_with;
use techalib::{
    errors::TechalibError,
    indicators::adx::{adx, adx_into, lookback_from_period, AdxResult, AdxSample},
    traits::State,
    types::Float,
};

fn generated_and_no_lookahead_adx(file_name: &str, period: usize) {
    let columns = load_generated_csv(file_name).unwrap();

    let high = columns.get("high").unwrap();
    let low = columns.get("low").unwrap();
    let close = columns.get("close").unwrap();
    let len = high.len();

    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected_adx = columns.get("adx").unwrap();

    let output = adx(
        &high[0..last_idx],
        &low[0..last_idx],
        &close[0..last_idx],
        period,
    );
    assert!(
        output.is_ok(),
        "Failed to calculate ADX: {:?}",
        output.err()
    );
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected_adx[0..last_idx], &result.adx);

    let mut new_state = result.state;
    for i in 0..next_count {
        let sample = AdxSample {
            high: high[last_idx + i],
            low: low[last_idx + i],
            close: close[last_idx + i],
        };
        let ok = new_state.update(&sample);
        assert!(ok.is_ok());
        assert!(
            approx_eq_float(new_state.prev_adx, expected_adx[last_idx + i], 1e-8),
            "Next [{}] expected {} but got {}",
            i,
            expected_adx[last_idx + i],
            new_state.prev_adx
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_adx("adx.csv", 14)
}

#[test]
fn generated_with_no_lookahead_period_3_ok() {
    generated_and_no_lookahead_adx("adx_timeperiod-3.csv", 3)
}

#[test]
fn finite_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![
        Float::MAX - 3.0,
        Float::MAX - 2.0,
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
    ];
    let period = 2;
    expect_err_overflow_or_ok_with!(adx(&data, &data, &data, period), |result: AdxResult| {
        assert!(
            result
                .adx
                .iter()
                .skip(lookback_from_period(period).unwrap())
                .all(|v| v.is_finite()),
            "Expected all values to be finite {:?}",
            result.adx
        );
    });
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let period = 3;
    let result = adx(&data, &data, &data, period).unwrap();
    let mut state = result.state;
    let sample = AdxSample {
        high: Float::MIN + 5.0,
        low: Float::MIN + 5.0,
        close: Float::MIN + 5.0,
    };
    expect_err_overflow_or_ok_with!(state.update(&sample), |_| {
        assert!(
            state.prev_adx.is_finite(),
            "Expected all values to be finite"
        );
    });
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let period = 3;
    let result = adx(&data, &data, &data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let data = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let period = 3;
    let result = adx(&data, &data, &data, period);
    assert!(
        result.is_err(),
        "Expected an error for non-finite data, got: {:?}",
        result
    );
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn empty_input_err() {
    let data: [Float; 0] = [];
    let period = 3;
    let result = adx(&data, &data, &data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::InsufficientData)));
}

#[test]
fn different_length_input_output_err() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let period = 3;
    let mut output = vec![0.0; 3];
    let result = adx_into(&data, &data, &data, period, output.as_mut_slice());
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}
