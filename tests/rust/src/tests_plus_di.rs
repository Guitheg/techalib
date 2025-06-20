use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use crate::expect_err_overflow_or_ok_with;
use techalib::{
    errors::TechalibError,
    indicators::plus_di::{plus_di, plus_di_into, PlusDiResult, PlusDiSample},
    traits::State,
    types::Float,
};

fn generated_and_no_lookahead_plus_di(file_name: &str, period: usize) {
    let columns = load_generated_csv(file_name).unwrap();

    let high = columns.get("high").unwrap();
    let low = columns.get("low").unwrap();
    let close = columns.get("close").unwrap();
    let len = high.len();

    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected_plus_di = columns.get("plus_di").unwrap();

    let output = plus_di(
        &high[0..last_idx],
        &low[0..last_idx],
        &close[0..last_idx],
        period,
    );
    assert!(
        output.is_ok(),
        "Failed to calculate PLUS_DI: {:?}",
        output.err()
    );
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected_plus_di[0..last_idx], &result.plus_di);

    let mut new_state = result.state;
    for i in 0..next_count {
        let sample = PlusDiSample {
            high: high[last_idx + i],
            low: low[last_idx + i],
            close: close[last_idx + i],
        };
        let ok = new_state.update(sample);
        assert!(ok.is_ok());
        assert!(
            approx_eq_float(new_state.prev_plus_di, expected_plus_di[last_idx + i], 1e-8),
            "Next [{}] expected {} but got {}",
            i,
            expected_plus_di[last_idx + i],
            new_state.prev_plus_di
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_plus_di("plus_di.csv", 14)
}

#[test]
fn generated_with_no_lookahead_period_1_ok() {
    generated_and_no_lookahead_plus_di("plus_di_timeperiod-1.csv", 1)
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
    let period = 3;
    expect_err_overflow_or_ok_with!(
        plus_di(&data, &data, &data, period),
        |result: PlusDiResult| {
            assert!(
                result.plus_di.iter().skip(period).all(|v| v.is_finite()),
                "Expected all values to be finite"
            );
        }
    );
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let period = 3;
    let result = plus_di(&data, &data, &data, period).unwrap();
    let mut state = result.state;
    let sample = PlusDiSample {
        high: Float::MIN + 5.0,
        low: Float::MIN + 5.0,
        close: Float::MIN + 5.0,
    };
    expect_err_overflow_or_ok_with!(state.update(sample), |_| {
        assert!(
            state.prev_plus_di.is_finite(),
            "Expected all values to be finite"
        );
    });
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let period = 3;
    let result = plus_di(&data, &data, &data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let data = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let period = 3;
    let result = plus_di(&data, &data, &data, period);
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
    let result = plus_di(&data, &data, &data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::InsufficientData)));
}

#[test]
fn different_length_input_output_err() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let period = 3;
    let mut output = vec![0.0; 3];
    let result = plus_di_into(&data, &data, &data, period, output.as_mut_slice());
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}
