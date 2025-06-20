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
    Inspired by TA-LIB AROONOSC implementation
*/

//! Aroon Oscillator (AROONOSC) implementation

use std::collections::VecDeque;

use crate::errors::TechalibError;
use crate::indicators::aroon;
use crate::traits::State;
use crate::types::Float;

/// AROONOSC calculation result
/// ---
/// This struct holds the result and the state ([`AroonoscState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `aroonosc`: A vector of [`Float`] containing the calculated AROONOSC values.
/// - `state`: A [`AroonoscState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct AroonoscResult {
    /// The AROONOSC values calculated
    pub aroonosc: Vec<Float>,
    /// The [`AroonoscState`] state of the AROONOSC calculation.
    pub state: AroonoscState,
}

/// AROONOSC calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Previous outputs values**
/// - `prev_aroonosc`: The previous AROONOSC value.
///
/// **State values**
/// - `prev_high_window`: A deque containing the previous high prices window.
/// - `prev_low_window`: A deque containing the previous low prices window.
///
/// **Parameters**
/// - `period`: The period used for the AROONOSC calculation.
#[derive(Debug, Clone)]
pub struct AroonoscState {
    // Outputs
    /// The previous AROONOSC value
    pub prev_aroonosc: Float,
    // State values
    /// The previous high prices window
    pub prev_high_window: VecDeque<Float>,
    /// The previous low prices window
    pub prev_low_window: VecDeque<Float>,
    // Parameters
    /// The period used for the AROONOSC calculation
    pub period: usize,
}

/// AROONOSC sample
/// ---
/// This struct represents a sample for the AROONOSC calculation.
/// It contains the high and low prices of the sample.
#[derive(Debug, Clone, Copy)]
pub struct AroonoscSample {
    /// The high price of the sample
    pub high: Float,
    /// The low price of the sample
    pub low: Float,
}

impl State<AroonoscSample> for AroonoscState {
    /// Update the [`AroonoscState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the AROONOSC state
    fn update(&mut self, sample: AroonoscSample) -> Result<(), TechalibError> {
        check_finite!(sample.high, sample.low);
        check_vec_finite!(self.prev_high_window);
        check_vec_finite!(self.prev_low_window);
        check_param_eq!(self.prev_high_window.len(), self.period);
        check_param_eq!(self.prev_low_window.len(), self.period);
        let mut high_window = self.prev_high_window.clone();
        let mut low_window = self.prev_low_window.clone();

        high_window.push_back(sample.high);
        low_window.push_back(sample.low);

        let new_aroonosc = aroonosc_next_unchecked(
            high_window.make_contiguous(),
            low_window.make_contiguous(),
            100.0 / self.period as Float,
        );

        high_window.pop_front();
        low_window.pop_front();

        check_finite!(&new_aroonosc);
        self.prev_aroonosc = new_aroonosc;
        self.prev_high_window = high_window;
        self.prev_low_window = low_window;
        Ok(())
    }
}

/// Lookback period for AROONOSC calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    check_param_gte!(period, 2);
    Ok(period)
}

/// Calculation of the AROONOSC function
/// ---
/// It returns a [`AroonoscResult`]
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `period`: The period used for the AROONOSC calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`AroonoscResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn aroonosc(
    high: &[Float],
    low: &[Float],
    period: usize,
) -> Result<AroonoscResult, TechalibError> {
    let mut output_aroonosc = vec![0.0; high.len()];

    let aroonosc_state = aroonosc_into(high, low, period, output_aroonosc.as_mut_slice())?;

    Ok(AroonoscResult {
        aroonosc: output_aroonosc,
        state: aroonosc_state,
    })
}

/// Calculation of the AROONOSC function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`AroonoscState`].
///
/// Input Arguments
/// ---
/// - `high`: A slice of [`Float`] representing the high prices.
/// - `low`: A slice of [`Float`] representing the low prices.
/// - `period`: The period used for the AROONOSC calculation.
///
/// Output Arguments
/// ---
/// - `output_aroonosc`: A mutable slice of [`Float`] where the AROONOSC values will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`AroonoscState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn aroonosc_into(
    high: &[Float],
    low: &[Float],
    period: usize,
    output_aroonosc: &mut [Float],
) -> Result<AroonoscState, TechalibError> {
    check_param_eq!(high.len(), low.len());
    check_param_eq!(high.len(), output_aroonosc.len());
    let len = high.len();

    let lookback = lookback_from_period(period)?;
    let factor = 100.0 / period as Float;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    let first_output_aroonosc =
        init_aroonosc_unchecked(high, low, factor, lookback, output_aroonosc)?;
    output_aroonosc[lookback] = first_output_aroonosc;
    check_finite_at!(lookback, output_aroonosc);

    for idx in lookback + 1..len {
        check_finite_at!(idx, high, low);

        output_aroonosc[idx] =
            aroonosc_next_unchecked(&high[idx - period..=idx], &low[idx - period..=idx], factor);

        check_finite_at!(idx, output_aroonosc);
    }

    Ok(AroonoscState {
        prev_aroonosc: output_aroonosc[len - 1],
        prev_high_window: VecDeque::from(high[len - period..len].to_vec()),
        prev_low_window: VecDeque::from(low[len - period..len].to_vec()),
        period,
    })
}

#[inline(always)]
fn init_aroonosc_unchecked(
    high: &[Float],
    low: &[Float],
    factor: Float,
    lookback: usize,
    output_aroonosc: &mut [Float],
) -> Result<Float, TechalibError> {
    let mut _unused_output = vec![0.0; lookback + 1];
    let (aroon_down, aroon_up) = aroon::init_aroon_unchecked(
        high,
        low,
        factor,
        lookback,
        output_aroonosc,
        &mut _unused_output,
    )?;
    Ok(aroon_up - aroon_down)
}

#[inline(always)]
fn aroonosc_next_unchecked(high_window: &[Float], low_window: &[Float], factor: Float) -> Float {
    let (aroon_down, aroon_up) = aroon::aroon_next_unchecked(high_window, low_window, factor);
    aroon_up - aroon_down
}
