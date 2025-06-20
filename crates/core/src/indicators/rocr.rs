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
    Inspired by TA-LIB ROCR implementation
*/

//! Rate of Change Ratio (ROCR) implementation

use std::collections::VecDeque;

use crate::errors::TechalibError;
use crate::traits::State;
use crate::types::Float;

/// ROCR calculation result
/// ---
/// This struct holds the result and the state ([`RocrState`])
/// of the calculation.
///
/// Attributes
/// ---
/// - `rocr`: A vector of [`Float`] representing the calculated values.
/// - `state`: A [`RocrState`], which can be used to calculate
///   the next values incrementally.
#[derive(Debug)]
pub struct RocrResult {
    /// The calculated ROCR values.
    pub rocr: Vec<Float>,
    /// The [`RocrState`] state of the ROCR calculation.
    pub state: RocrState,
}

/// ROCR calculation state
/// ---
/// This struct holds the state of the calculation.
/// It is used to calculate the next values in a incremental way.
///
/// Attributes
/// ---
/// **Previous outputs values**
/// - `prev_rocr`: The last calculated ROCR value.
///
/// **State values**
/// - `prev_roc_window`: The previous values used for the ROCR calculation.
///
/// **Parameters**
/// - `period`: The period for the ROCR calculation.
#[derive(Debug, Clone)]
pub struct RocrState {
    // Outputs
    /// The last calculated ROCR value.
    pub prev_rocr: Float,
    // State values
    /// The previous values used for the ROCR calculation.
    pub prev_roc_window: VecDeque<Float>,
    // Parameters
    /// The period for the ROCR calculation.
    pub period: usize,
}

impl State<Float> for RocrState {
    /// Update the [`RocrState`] with a new sample
    ///
    /// Input Arguments
    /// ---
    /// - `sample`: The new input to update the ROCR state
    fn update(&mut self, sample: Float) -> Result<(), TechalibError> {
        check_finite!(sample);
        check_param_eq!(self.prev_roc_window.len(), self.period);
        check_vec_finite!(self.prev_roc_window);

        let mut window = self.prev_roc_window.clone();

        let old_value = window.pop_front().ok_or(TechalibError::InsufficientData)?;
        window.push_back(sample);

        let new_rocr = rocr_next_unchecked(sample, old_value);

        check_finite!(new_rocr);
        self.prev_rocr = new_rocr;
        self.prev_roc_window = window;
        Ok(())
    }
}

/// Lookback period for ROCR calculation
/// ---
/// With `n = lookback_from_period(period)`,
/// the `n-1` first values that will be return will be `NaN`
/// The n-th value will be the first valid value,
#[inline(always)]
pub fn lookback_from_period(period: usize) -> Result<usize, TechalibError> {
    check_param_gte!(period, 1);
    Ok(period)
}

/// Calculation of the ROCR function
/// ---
/// It returns a [`RocrResult`]
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the calculation.
///
/// Returns
/// ---
/// A `Result` containing a [`RocrResult`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn rocr(data: &[Float], period: usize) -> Result<RocrResult, TechalibError> {
    let mut output = vec![0.0; data.len()];

    let rocr_state = rocr_into(data, period, output.as_mut_slice())?;

    Ok(RocrResult {
        rocr: output,
        state: rocr_state,
    })
}

/// Calculation of the ROCR function
/// ---
/// It stores the results in the provided output arrays and
/// return the state [`RocrState`].
///
/// Input Arguments
/// ---
/// - `data`: A slice of [`Float`] representing the input data.
/// - `period`: The period for the calculation.
///
/// Output Arguments
/// ---
/// - `output`: A mutable slice of [`Float`] where the results will be stored.
///
/// Returns
/// ---
/// A `Result` containing a [`RocrState`],
/// or a [`TechalibError`] error if the calculation fails.
pub fn rocr_into(
    data: &[Float],
    period: usize,
    output: &mut [Float],
) -> Result<RocrState, TechalibError> {
    check_param_eq!(data.len(), output.len());
    let len = data.len();

    let lookback = lookback_from_period(period)?;

    if len <= lookback {
        return Err(TechalibError::InsufficientData);
    }

    let first_output = init_rocr_unchecked(data, lookback, output)?;
    check_finite!(first_output);
    output[lookback] = first_output;

    for idx in lookback + 1..len {
        check_finite_at!(idx, data);

        output[idx] = rocr_next_unchecked(data[idx], data[idx - period]);

        check_finite_at!(idx, output);
    }

    Ok(RocrState {
        prev_rocr: output[len - 1],
        prev_roc_window: VecDeque::from(data[len - period..len].to_vec()),
        period,
    })
}

#[inline(always)]
fn init_rocr_unchecked(
    data: &[Float],
    lookback: usize,
    output: &mut [Float],
) -> Result<Float, TechalibError> {
    for idx in 0..lookback {
        check_finite_at!(idx, data);
        output[idx] = Float::NAN;
    }
    check_finite_at!(lookback, data);
    Ok(rocr_next_unchecked(data[lookback], data[0]))
}

#[inline(always)]
fn rocr_next_unchecked(new_value: Float, prev_value: Float) -> Float {
    if prev_value == 0.0 {
        return 0.0;
    }
    new_value / prev_value
}
