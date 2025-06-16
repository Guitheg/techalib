use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use crate::expect_err_overflow_or_ok_with;
use techalib::{
    errors::TechalibError,
    indicators::atr::{atr, atr_into, AtrResult, AtrSample},
    traits::State,
    types::Float,
};

fn generated_and_no_lookahead_atr(file_name: &str, period: usize) {
    let columns = load_generated_csv(file_name).unwrap();
    let close = columns.get("close").unwrap();
    let high = columns.get("high").unwrap();
    let low = columns.get("low").unwrap();

    let len = close.len();
    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected = columns.get("out").unwrap();

    let close_prev = &close[0..last_idx];
    let high_prev = &high[0..last_idx];
    let low_prev = &low[0..last_idx];

    let output = atr(high_prev, low_prev, close_prev, period);
    assert!(
        output.is_ok(),
        "Failed to calculate ATR: {:?}",
        output.err()
    );
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected[0..last_idx], &result.values);

    let mut new_state = result.state;
    for i in 0..next_count {
        let sample = AtrSample {
            high: high[last_idx + i],
            low: low[last_idx + i],
            close: close[last_idx + i],
        };
        new_state.update(&sample).unwrap();
        assert!(
            approx_eq_float(new_state.atr, expected[last_idx + i], 1e-8),
            "Next [{}] expected {}, but got {}",
            i,
            expected[last_idx + i],
            new_state.atr
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_atr("atr.csv", 14)
}

#[test]
fn generated_with_no_lookahead_period_1_ok() {
    generated_and_no_lookahead_atr("atr_timeperiod-1.csv", 1)
}

#[test]
fn finite_extreme_err_overflow_or_ok_all_finite() {
    let close = vec![
        Float::MAX - 3.0,
        Float::MAX - 2.0,
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
    ];
    let high = vec![
        Float::MAX - 3.0,
        Float::MAX - 2.0,
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
    ];
    let low = vec![
        Float::MAX - 3.0,
        Float::MAX - 2.0,
        Float::MAX - 5.0,
        Float::MAX - 6.0,
        Float::MAX - 8.0,
        Float::MAX - 1.0,
    ];
    let period = 3;
    expect_err_overflow_or_ok_with!(atr(&high, &low, &close, period), |result: AtrResult| {
        assert!(
            result.values.iter().skip(period).all(|v| v.is_finite()),
            "Expected all values to be finite"
        );
    });
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let close = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let high = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let low = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let period = 3;
    let result = atr(&high, &low, &close, period).unwrap();
    let mut state = result.state;
    let sample = AtrSample {
        high: Float::MIN + 5.0,
        low: Float::MIN + 2.0,
        close: Float::MIN + 3.0,
    };
    expect_err_overflow_or_ok_with!(state.update(&sample), |_| {
        assert!(state.atr.is_finite(), "Expected all values to be finite");
    });
}

#[test]
fn unexpected_nan_err() {
    let close = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let high = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let low = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let period = 3;
    let result = atr(&high, &low, &close, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let close = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let high = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let low = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let period = 3;
    let result = atr(&high, &low, &close, period);
    assert!(
        result.is_err(),
        "Expected an error for non-finite data, got: {:?}",
        result
    );
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn empty_input_err() {
    let close: [Float; 0] = [];
    let high: [Float; 0] = [];
    let low: [Float; 0] = [];
    let period = 14;
    let result = atr(&high, &low, &close, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::InsufficientData)));
}

#[test]
fn different_length_input_output_err() {
    let close = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut output = vec![0.0; 3];
    let period = 3;
    let result = atr_into(&close, &close, &close, period, output.as_mut_slice());
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}

#[test]
fn different_length_inputs_err() {
    let close = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let low = vec![1.0, 2.0, 3.0];
    let mut output = vec![0.0; 5];
    let period = 3;
    let result = atr_into(&close, &low, &close, period, output.as_mut_slice());
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}
