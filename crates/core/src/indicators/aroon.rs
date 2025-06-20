/*
    BSD 3-Clause License

    Copyright (c) 2025, Guillaume GOBIN (Guitheg)

    Redistribution and use in source and binary forms, with or without modification,
    are permitted provided that the following conditions are met:

    1. Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimer.

    2. Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions and the following disclaimer in the documentation and/or
    other materials provided with the distribution.

    3. Neither the name of the copyright holder nor the names of its contributors
    may be used to endorse or promote products derived from this software without
    specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
    WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
    THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

/*
    List of contributors:
    - Guitheg: Initial implementation
*/

/*
    Inspired by TA-LIB AROON implementation
*/

//! Aroon Up and Aroon Down Indicator (AROON) implementation

use std::collections::VecDeque;

use crate::errors::TechalibError;
use crate::traits::State;
use crate::types::Float;

/// AROON calculation result
/// ---
/// This struct holds the result and the state ([`AroonState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `aroon_down`: A vector of [`Float`] containing the AROON Down values.
/// - `aroon_up`: A vector of [`Float`] containing the AROON Up
/// - `state`: A [`AroonState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct AroonResult {
    /// The AROON Down values
    pub aroon_down: Vec<Float>,
    /// The AROON Up values
    pub aroon_up: Vec<Float>,
    /// The [`AroonState`] state of the AROON calculation.
    pub state: AroonState,
}

/// AROON calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Previous outputs values**
/// - `prev_aroon_down`: The previous AROON Down value.
/// - `prev_aroon_up`: The previous AROON Up value.
///
/// **State values**
/// - `prev_high_window`: A window of the previous high prices.
/// - `prev_low_window`: A window of the previous low prices.
///
/// **Parameters**
/// - `period`: The period used for the AROON calculation.
#[derive(Debug, Clone)]
pub struct AroonState {
    // Outputs
    /// The previous AROON Down value
    pub prev_aroon_down: Float,
    /// The previous AROON Up value
    pub prev_aroon_up: Float,
    // State values
    /// The previous high prices window
    pub prev_high_window: VecDeque<Float>,
    /// The previous low prices window
    pub prev_low_window: VecDeque<Float>,
    // Parameters
    /// The period used for the AROON calculation
    pub period: usize,
}

/// AROON sample
/// ---
/// This struct represents a sample for the AROON calculation.
/// It contains the high and low prices of the sample.
#[derive(Debug, Clone, Copy)]
pub struct AroonSample {
    /// The high price of the sample
    pub high: Float,
    /// The low price of the sample
    pub low: Float,
}

impl State<AroonSample> for AroonState {
    /// Update the [`AroonState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the AROON state
    fn update(&mut self, sample: AroonSample) -> Result<(), TechalibError> {
        check_finite!(sample.high, sample.low);
        check_vec_finite!(self.prev_high_window);
        check_vec_finite!(self.prev_low_window);
        check_param_eq!(self.prev_high_window.len(), self.period);
        check_param_eq!(self.prev_low_window.len(), self.period);

        let mut high_window = self.prev_high_window.clone();
        let mut low_window = self.prev_low_window.clone();

        high_window.push_back(sample.high);
        low_window.push_back(sample.low);

        let (new_aroon_down, new_aroon_up) = aroon_next_unchecked(
            high_window.make_contiguous(),
            low_window.make_contiguous(),
            100.0 / self.period as Float,
        );

        high_window.pop_front();
        low_window.pop_front();

        check_finite!(&new_aroon_down);
        self.prev_aroon_down = new_aroon_down;
        check_finite!(&new_aroon_up);
        self.prev_aroon_up = new_aroon_up;
        self.prev_high_window = high_window;
        self.prev_low_window = low_window;
        Ok(())
    }
}

/// Lookback period for AROON calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    check_param_gte!(period, 2);
    Ok(period)
}

/// Calculation of the AROON function
/// ---
/// It returns a [`AroonResult`]
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `period`: The period used for the AROON calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`AroonResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn aroon(high: &[Float], low: &[Float], period: usize) -> Result<AroonResult, TechalibError> {
    let mut output_aroon_down = vec![0.0; high.len()];
    let mut output_aroon_up = vec![0.0; high.len()];

    let aroon_state = aroon_into(
        high,
        low,
        period,
        output_aroon_down.as_mut_slice(),
        output_aroon_up.as_mut_slice(),
    )?;

    Ok(AroonResult {
        aroon_down: output_aroon_down,
        aroon_up: output_aroon_up,
        state: aroon_state,
    })
}

/// Calculation of the AROON function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`AroonState`].
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `period`: The period used for the AROON calculation.
///
/// Output Arguments
/// ---
/// - `output_aroon_down`: A mutable slice of [`Float`] to store the AROON Down values.
/// - `output_aroon_up`: A mutable slice of [`Float`] to store the AROON Up values.
///
/// Returns
/// ---
/// A `Result` containing a [`AroonState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn aroon_into(
    high: &[Float],
    low: &[Float],
    period: usize,
    output_aroon_down: &mut [Float],
    output_aroon_up: &mut [Float],
) -> Result<AroonState, TechalibError> {
    check_param_eq!(high.len(), low.len());
    check_param_eq!(high.len(), output_aroon_down.len());
    check_param_eq!(high.len(), output_aroon_up.len());
    let len = high.len();

    let lookback = lookback_from_period(period)?;
    let factor = 100.0 / period as Float;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    let (first_output_aroon_down, first_output_aroon_up) = init_aroon_unchecked(
        high,
        low,
        factor,
        lookback,
        output_aroon_down,
        output_aroon_up,
    )?;
    output_aroon_down[lookback] = first_output_aroon_down;
    check_finite_at!(lookback, output_aroon_down);
    output_aroon_up[lookback] = first_output_aroon_up;
    check_finite_at!(lookback, output_aroon_up);

    for idx in lookback + 1..len {
        check_finite_at!(idx, high, low);

        (output_aroon_down[idx], output_aroon_up[idx]) =
            aroon_next_unchecked(&high[idx - period..=idx], &low[idx - period..=idx], factor);

        check_finite_at!(idx, output_aroon_down, output_aroon_up);
    }

    Ok(AroonState {
        prev_aroon_down: output_aroon_down[len - 1],
        prev_aroon_up: output_aroon_up[len - 1],
        prev_high_window: VecDeque::from(high[len - period..len].to_vec()),
        prev_low_window: VecDeque::from(low[len - period..len].to_vec()),
        period,
    })
}

#[inline(always)]
pub(crate) fn init_aroon_unchecked(
    high: &[Float],
    low: &[Float],
    factor: Float,
    lookback: usize,
    output_aroon_down: &mut [Float],
    output_aroon_up: &mut [Float],
) -> Result<(Float, Float), TechalibError> {
    check_finite_at!(0, high);
    check_finite_at!(0, low);
    let mut maximum = high[0];
    let mut minimum = low[0];
    let mut max_idx = 0;
    let mut min_idx = 0;
    for idx in 0..=lookback {
        check_finite_at!(idx, high, low);
        ((maximum, max_idx), (minimum, min_idx)) =
            arg_maxmin(high[idx], low[idx], maximum, max_idx, minimum, min_idx, idx);
        output_aroon_down[idx] = f64::NAN;
        output_aroon_up[idx] = f64::NAN;
    }

    Ok((
        calculate_aroon(min_idx, factor),
        calculate_aroon(max_idx, factor),
    ))
}

#[inline(always)]
pub(crate) fn aroon_next_unchecked(
    high_window: &[Float],
    low_window: &[Float],
    factor: Float,
) -> (Float, Float) {
    let mut maximum = high_window[0];
    let mut minimum = low_window[0];
    let mut max_idx = 0;
    let mut min_idx = 0;
    for idx in 0..high_window.len() {
        ((maximum, max_idx), (minimum, min_idx)) = arg_maxmin(
            high_window[idx],
            low_window[idx],
            maximum,
            max_idx,
            minimum,
            min_idx,
            idx,
        );
    }
    (
        calculate_aroon(min_idx, factor),
        calculate_aroon(max_idx, factor),
    )
}

#[inline(always)]
pub(crate) fn arg_maxmin(
    high_value: Float,
    low_value: Float,
    maximum: Float,
    max_idx: usize,
    minimum: Float,
    min_idx: usize,
    idx: usize,
) -> ((Float, usize), (Float, usize)) {
    (
        if high_value > maximum {
            (high_value, idx)
        } else {
            (maximum, max_idx)
        },
        if low_value < minimum {
            (low_value, idx)
        } else {
            (minimum, min_idx)
        },
    )
}

#[inline(always)]
fn calculate_aroon(extremum_index: usize, factor: Float) -> Float {
    factor * extremum_index as Float
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eq_with_nan_eq(a: f64, b: f64) -> bool {
        (a.is_nan() && b.is_nan()) || (a == b)
    }

    fn vec_compare(va: &[f64], vb: &[f64]) -> bool {
        (va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| eq_with_nan_eq(*a, *b))
    }

    #[test]
    fn nominal_case() {
        let high = [10.0, 9.0, 8.0, 11.0, 12.0, 10.0, 9.5, 10.5, 11.5, 12.5];
        let low = [5.0, 4.0, 3.0, 6.0, 7.0, 5.5, 4.5, 5.5, 6.5, 7.5];
        let expected_down = [
            Float::NAN,
            Float::NAN,
            Float::NAN,
            200.0 / 3.0,
            100.0 / 3.0,
            0.0,
            100.0,
            200.0 / 3.0,
            100.0 / 3.0,
            0.0,
        ];
        let expected_up = [
            Float::NAN,
            Float::NAN,
            Float::NAN,
            100.0,
            100.0,
            200.0 / 3.0,
            100.0 / 3.0,
            0.0,
            100.0,
            100.0,
        ];
        let period = 3;
        let result = aroon(&high, &low, period).unwrap();
        assert!(
            vec_compare(&result.aroon_down, &expected_down),
            "down: Expected:\t{:?},\ngot:\t\t{:?}",
            expected_down,
            result.aroon_down,
        );
        assert!(
            vec_compare(&result.aroon_up, &expected_up),
            "up: Expected:\t{:?},\ngot:\t\t{:?}",
            expected_up,
            result.aroon_up,
        );
    }
}
