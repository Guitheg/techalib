use crate::helper::{
    assert::approx_eq_float,
    generated::{assert_vec_eq_gen_data, load_generated_csv},
};

use crate::expect_err_overflow_or_ok_with;
use techalib::{
    errors::TechalibError,
    indicators::rocr::{rocr, rocr_into, RocrResult},
    traits::State,
    types::Float,
};

fn generated_and_no_lookahead_rocr(file_name: &str, period: usize) {
    let columns = load_generated_csv(file_name).unwrap();

    let input = columns.get("close").unwrap();
    let len = input.len();

    let next_count = 5;
    let last_idx = len - (1 + next_count);

    let expected = columns.get("rocr").unwrap();

    let output = rocr(&input[0..last_idx], period);
    assert!(
        output.is_ok(),
        "Failed to calculate ROCR: {:?}",
        output.err()
    );
    let result = output.unwrap();

    assert_vec_eq_gen_data(&expected[0..last_idx], &result.rocr);

    let mut new_state = result.state;
    for i in 0..next_count {
        let sample = input[last_idx + i];
        let ok = new_state.update(sample);
        assert!(ok.is_ok());
        assert!(
            approx_eq_float(new_state.prev_rocr, expected[last_idx + i], 1e-8),
            "Next [{}] expected {} but got {}",
            i,
            expected[last_idx + i],
            new_state.prev_rocr
        );
    }
}

#[test]
fn generated_with_no_lookahead_ok() {
    generated_and_no_lookahead_rocr("rocr.csv", 14)
}

#[test]
fn generated_with_no_lookahead_period_1_ok() {
    generated_and_no_lookahead_rocr("rocr_timeperiod-1.csv", 1)
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
    expect_err_overflow_or_ok_with!(rocr(&data, period), |result: RocrResult| {
        assert!(
            result.rocr.iter().skip(period).all(|v| v.is_finite()),
            "Expected all values to be finite"
        );
    });
}

#[test]
fn next_with_finite_neg_extreme_err_overflow_or_ok_all_finite() {
    let data = vec![5.0, 10.0, 30.0, 3.0, 5.0, 6.0, 8.0];
    let result = rocr(&data, 3).unwrap();
    let mut state = result.state;
    let sample = Float::MIN + 5.0;
    expect_err_overflow_or_ok_with!(state.update(sample), |_| {
        assert!(
            state.prev_rocr.is_finite(),
            "Expected all values to be finite"
        );
    });
}

#[test]
fn unexpected_nan_err() {
    let data = vec![1.0, 2.0, 3.0, Float::NAN, 1.0, 2.0, 3.0];
    let result = rocr(&data, 3);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::DataNonFinite(_))));
}

#[test]
fn non_finite_err() {
    let data = vec![1.0, 2.0, Float::INFINITY, 1.0, 2.0, 3.0];
    let period = 3;
    let result = rocr(&data, period);
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
    let result = rocr(&data, period);
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::InsufficientData)));
}

#[test]
fn different_length_input_output_err() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let period = 3;
    let mut output = vec![0.0; 3];
    let result = rocr_into(&data, period, output.as_mut_slice());
    assert!(result.is_err());
    assert!(matches!(result, Err(TechalibError::BadParam(_))));
}
